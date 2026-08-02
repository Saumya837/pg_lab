// =============================================================================
// GiST INDEX FOR Vector2D
// =============================================================================
//
// WHAT WE ARE BUILDING:
//   An index that makes spatial queries fast on 2D points.
//
// WITHOUT THIS INDEX:
//   SELECT * FROM locations WHERE pos <-> '(0,0)'::vector2d < 5.0;
//   → Postgres scans EVERY row, computes distance for each one
//   → 1 million rows = 1 million distance calculations
//
// WITH THIS INDEX:
//   → Postgres prunes entire subtrees that can't possibly contain matches
//   → 1 million rows = maybe 50 distance calculations
//
// THE QUERY WE ARE ENABLING:
//   -- Find all points within radius 5 of origin
//   SELECT * FROM locations WHERE pos <@ circle_query('(0,0)', 5.0);
//
//   -- Find nearest 10 points to a location (KNN - later)
//   SELECT * FROM locations ORDER BY pos <-> '(3,4)'::vector2d LIMIT 10;
//
// =============================================================================
// THE BOUNDING BOX — THE HEART OF GIST
// =============================================================================
//
// GiST does NOT store individual points in internal nodes.
// It stores BOUNDING BOXES — rectangles that cover all points below them.
//
// Example tree with 6 points:
//
//                    ┌─────────────────────────┐
//                    │  BBox(0,0)→(9,9)        │  ← ROOT: covers everything
//                    └────────┬────────┬────────┘
//                             │        │
//              ┌──────────────┘        └──────────────┐
//              ▼                                       ▼
//   ┌─────────────────┐                   ┌─────────────────┐
//   │  BBox(0,0)→(4,5)│                   │  BBox(5,1)→(9,9)│
//   └──┬──────────┬───┘                   └──┬──────────┬───┘
//      │          │                          │          │
//   (1,2)       (3,4)                      (6,3)       (8,7)
//   (2,5)                                  (9,1)
//
// When you query "find points inside circle((0,0), 5)":
//
//   1. Check root BBox(0,0)→(9,9) — does it overlap circle? YES → go deeper
//   2. Check BBox(0,0)→(4,5) — does it overlap circle? YES → go deeper
//      Check (1,2) → inside circle? YES → RETURN IT
//      Check (3,4) → inside circle? YES → RETURN IT
//      Check (2,5) → inside circle? NO → skip
//   3. Check BBox(5,1)→(9,9) — does it overlap circle? MAYBE → go deeper
//      Check (6,3) → inside circle? NO → skip
//      Check (8,7) → inside circle? NO → skip
//      Check (9,1) → inside circle? NO → skip
//
// Result: checked 6 points but only visited 2 subtrees
// Without index: would check all 6 points regardless
// At 1 million points the savings are enormous
//
// =============================================================================

use pgrx::prelude::*;
use pgrx::pg_sys;


// Your Vector2D type 
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

// =============================================================================
// BOUNDING BOX — what GiST stores in internal nodes
//
// REQUIREMENT: Must be able to represent any rectangular region in 2D space
// REQUIREMENT: Must be able to expand to cover new points
// REQUIREMENT: Must handle degenerate case (single point = box of zero area)
//
// BEHAVIOR:
//   BoundingBox::from_point((3,4)) → BBox { min_x:3, max_x:3, min_y:4, max_y:4 }
//   bbox1.expand(bbox2) → smallest box containing both
//   bbox.overlaps_circle(center, radius) → true if any part of box is in circle
//   bbox.area() → width * height
// =============================================================================

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct BoundingBox {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}


#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct CircleQuery{
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl BoundingBox {
    // -------------------------------------------------------------------------
    // from_point
    //
    // REQUIREMENT: Create a bounding box from a single point
    // A single point has zero area — min == max on both axes
    //
    // BEHAVIOR:
    //   from_point(Vector2D { x: 3.0, y: 4.0 })
    //   → BoundingBox { min_x: 3.0, max_x: 3.0, min_y: 4.0, max_y: 4.0 }
    //
    // WHY: When GiST first indexes a point, it needs a bounding box for it.
    //      A point IS its own bounding box.
    // -------------------------------------------------------------------------
    pub fn from_point(p: &Vector2D) -> Self {
        Self
        {
            min_x: p.x,
            max_x: p.x,
            min_y: p.y,
            max_y: p.y
        }
    }

    // -------------------------------------------------------------------------
    // expand
    //
    // REQUIREMENT: Return the smallest box that contains BOTH self and other
    // REQUIREMENT: Must be commutative: a.expand(b) == b.expand(a)
    // REQUIREMENT: Must be associative: a.expand(b).expand(c) == a.expand(b.expand(c))
    //
    // BEHAVIOR:
    //   BBox(1,3,1,3).expand(BBox(2,5,0,2))
    //   → BBox(min_x:1, max_x:5, min_y:0, max_y:3)
    //
    //   Rule: take min of mins, max of maxes
    //
    // WHY: Used by UNION to build the bounding box of an internal node.
    //      Also used by PENALTY to calculate how much a box grows.
    // -------------------------------------------------------------------------
    pub fn expand(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox{
            min_x: self.min_x.min(other.min_x),
            max_x: self.max_x.max(other.max_x),
            min_y: self.min_y.min(other.min_y),
            max_y: self.max_y.max(other.max_y)
        }
    }

    // REQUIREMENT: Return width * height of the bounding box
    // REQUIREMENT: Single point (zero-area box) returns 0.0, not panic
    
    // BEHAVIOR:
    //   BBox(1,4,1,3).area() → (4-1) * (3-1) = 6.0
    //   BBox(3,3,4,4).area() → 0.0  (single point)

    // WHY: Used by PENALTY — we measure cost in terms of area expansion.

    pub fn area(&self) -> f64 {
        let width = self.max_x - self.min_x;
        let height = self.max_y - self.min_y;
        width * height
    }

    // expansion_cost

    // Return how much area is ADDED by expanding self to cover other
    // If other is already inside self, return 0.0
    // Never return negative
    
    // PENALTY uses this to decide which subtree to insert into.
    //      GiST picks the subtree whose box expands LEAST.

    pub fn expansion_cost(&self, other: &BoundingBox) -> f64 {
        let expanded = self.expand(other);
        expanded.area() - self.area()
    }

    // -------------------------------------------------------------------------
    // overlaps_circle
    //
    // REQUIREMENT: Return true if ANY part of the box is within the circle
    // REQUIREMENT: Points exactly on the circle boundary count as overlapping
    // REQUIREMENT: If box is entirely inside circle, return true
    // REQUIREMENT: If circle is entirely inside box, return true
    //
    // BEHAVIOR:
    //   BBox(0,2,0,2).overlaps_circle(center=(0,0), radius=1.5) → true
    //   BBox(5,8,5,8).overlaps_circle(center=(0,0), radius=1.5) → false
    //   BBox(-1,1,-1,1).overlaps_circle(center=(0,0), radius=5.0) → true (box inside circle)
    //   BBox(-10,10,-10,10).overlaps_circle(center=(0,0), radius=1.0) → true (circle inside box)
    //
    // ALGORITHM: Find the closest point on the box to the circle center.
    //   closest_x = clamp(center_x, min_x, max_x)
    //   closest_y = clamp(center_y, min_y, max_y)
    //   distance² = (center_x - closest_x)² + (center_y - closest_y)²
    //   overlaps if distance² <= radius²
    //   (no sqrt needed — comparing squares is equivalent and faster)
    //
    // WHY: Used by CONSISTENT on internal nodes.
    //      "Does this subtree's bounding box overlap the query circle?"
    //      If NO → prune entire subtree (huge speedup)
    //      If YES → go deeper
    // -------------------------------------------------------------------------
    pub fn overlaps_circle(&self, center_x: f64, center_y: f64, radius: f64) -> bool {
        let closest_x = center_x.clamp(self.min_x, self.max_x);
        let closest_y = center_y.clamp(self.min_y, self.max_y);

        let dx = center_x - closest_x;
        let dy = center_y - closest_y;

        (dx * dx) + (dy * dy) <= (radius * radius)
    }

    // -------------------------------------------------------------------------
    // contains_point
    //
    // REQUIREMENT: Return true if the point is inside or on the boundary of box
    //
    // BEHAVIOR:
    //   BBox(0,5,0,5).contains_point(Vector2D { x:3, y:3 }) → true
    //   BBox(0,5,0,5).contains_point(Vector2D { x:5, y:5 }) → true  (boundary)
    //   BBox(0,5,0,5).contains_point(Vector2D { x:6, y:3 }) → false
    //
    // WHY: Used by CONSISTENT on leaf nodes.
    //      At a leaf, we have the actual point, not a bounding box.
    //      "Is this actual point inside the query circle?"
    //      Wait — this is actually used differently. See CONSISTENT below.
    // -------------------------------------------------------------------------
    pub fn contains_point(&self, p: &Vector2D) -> bool {
        (self.min_x <= p.x && self.max_x >= p.x) && (self.min_y <= p.y && self.max_y >= p.y)
    }
}

// Helper: distance² between two points (no sqrt = faster)
fn dist_squared(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
}

// =============================================================================
// GIST SUPPORT FUNCTION 1: CONSISTENT
// =============================================================================
// WHAT IT DOES: Decides whether to search a subtree or prune it
//
// REQUIREMENT: Must never return false when the answer could be true
//              (false negatives are FORBIDDEN — you'd miss real results)
//              Returning true when false is OK (false positive = just extra work)
//
// false negative means it is present but showing not present
//
// REQUIREMENT: At LEAF nodes — check the ACTUAL POINT against the query
//              At INTERNAL nodes — check the BOUNDING BOX against the query
//
// WHY TWO DIFFERENT CHECKS?
//   Internal node stores a BOUNDING BOX — not an actual point.
//   The box is an approximation: "all real points are somewhere inside here."
//   We ask: "Could the circle overlap this box?" If no → prune.
//   If yes → go deeper, because there MIGHT be real points inside.
//
//   Leaf node stores the ACTUAL POINT.
//   We ask: "Is this exact point inside the circle?" Exact answer, no approximation.
//
//   Internal = conservative (can return true even if no match exists below)
//   Leaf     = exact (true means the point IS in the result set)
//
// BEHAVIOR:
//   Query: find points within radius 5 of (0,0)
//
//   Internal node with BBox(0,3,0,3):
//     overlaps_circle((0,0), 5)? YES → return true (go deeper)
//
//   Internal node with BBox(10,20,10,20):
//     overlaps_circle((0,0), 5)? NO → return false (PRUNE — don't go deeper)
//
//   Leaf node with point (3,4):
//     distance to (0,0) = 5.0, radius = 5.0 → return true (IS in result)
//
//   Leaf node with point (4,4):
//     distance to (0,0) = 5.65 > 5.0 → return false (NOT in result)
//
// =============================================================================


fn is_point_in_circle(point: &Vector2D, query: &CircleQuery) -> bool {
    dist_squared(point.x, point.y, query.x, query.y) <= (query.radius * query.radius) 
}

fn does_bbox_overlap_circle(bbox: &BoundingBox, query: &CircleQuery) -> bool {
    bbox.overlaps_circle(query.x, query.y, query.radius)
}

unsafe fn get_point_at(entryvec: *mut pg_sys::GistEntryVector, i:usize) -> Vector2D{
    let datum = (*entryvec).vector.as_ptr().add(i).as_ref().unwrap().key; //  beyond pg_15 replace this to { (*entryvec).vector[i].key; }
    *(datum.cast_mut_ptr::<Vector2D>())    //*(pg_sys::DatumGetPointer(datum)as *mut Vector2D) //datum pointer -> vector pointer
}

unsafe fn bbox_to_datum(bbox: BoundingBox) -> pg_sys::Datum {
    let ptr = pg_sys::palloc(std::mem::size_of::<BoundingBox>()) as *mut BoundingBox;
    *ptr = bbox;
    pg_sys::Datum::from(ptr as usize)
}

#[pg_guard]
pub unsafe extern "C-unwind" fn vector2d_gist_consistent(
    fcinfo: pg_sys::FunctionCallInfo,
) -> pg_sys::Datum {
    let entry = (*(*fcinfo).args.as_ptr()).value.cast_mut_ptr::<pg_sys::GISTENTRY>(); // beyond pg_15 replace this to { pg_sys::PG_GETARG_POINTER(fcinfo, 0) as *mut pg_sys::GISTENTRY };
    let query = (*(*fcinfo).args.as_ptr().add(1)).value.cast_mut_ptr::<CircleQuery>(); //beyond pg_15 replace this to { pg_sys::PG_GETARG_POINTER(fcinfo, 1) as *mut CircleQuery };   

    let result = if (*entry).leafkey { // beyond pg_15 replace this to {if pg_sys::GIST_LEAF(entry)}
        let point = (*entry).key.cast_mut_ptr::<Vector2D>();  //beyond pg_15 replace this to { pg_sys::DatumGetPointer((*entry).key) as *mut Vector2D };
        is_point_in_circle(&*point, &*query)
    } 
    else{
        let bbox = (*entry).key.cast_mut_ptr::<BoundingBox>();  //beyond pg_15 replace this to { pg_sys::DatumGetPointer((*entry).key) as *mut BoundingBox };
        does_bbox_overlap_circle(&*bbox, &*query)
    };
    
    pg_sys::Datum::from(result as usize)
}

// =============================================================================
// GIST SUPPORT FUNCTION 2: UNION
// =============================================================================
//
// WHAT IT DOES: Builds the bounding box for an internal node
//
// REQUIREMENT: Must return the SMALLEST box that contains ALL entries
// REQUIREMENT: Must handle 1 entry (trivially returns that entry's box)
// REQUIREMENT: Must handle N entries (iteratively expand)
//
// BEHAVIOR:
//   entries = [(1,2), (3,4), (5,1)]
//   union = BBox(min_x:1, max_x:5, min_y:1, max_y:4)
//
//   entries = [(BBox(0,2,0,2)), (BBox(3,5,1,4))]
//   union = BBox(min_x:0, max_x:5, min_y:0, max_y:4)
//
// WHY:
//   When Postgres builds or updates the GiST tree, internal nodes need
//   to store a key that summarizes all children below them.
//   UNION computes that summary.
//   Called during: INSERT, VACUUM, index rebuild
//
// =============================================================================
#[pg_guard]
pub unsafe extern "C-unwind" fn vector2d_gist_union(
    fcinfo: pg_sys::FunctionCallInfo,
) -> pg_sys::Datum {
    let entryvec = (*(*fcinfo).args.as_ptr()).value.cast_mut_ptr::<pg_sys::GistEntryVector>(); //pg_sys::PG_GETARG_POINTER(fcinfo, 0) as *mut pg_sys::GistEntryVector;
    let num_of_entries = (*entryvec).n;

    let first_key = (*(*entryvec).vector.as_ptr()).key.cast_mut_ptr::<BoundingBox>(); //pg_sys::DatumGetPointer((*entryvec).vector[0].key) as *mut BoundingBox;
    let mut result = *first_key;

    for i in 1..num_of_entries{
        let key =(*(*entryvec).vector.as_ptr().add(i as usize)).key.cast_mut_ptr::<BoundingBox>(); //pg_sys::DatumGetPointer((*entryvec).vector[i as usize].key) as *mut BoundingBox;
        result = result.expand(&*key);
    }

    let result_ptr = pg_sys::palloc(std::mem::size_of::<BoundingBox>()) as *mut BoundingBox;
    *result_ptr = result;

    pg_sys::Datum::from(result_ptr as usize)
}

// =============================================================================
// GIST SUPPORT FUNCTION 3: PENALTY
// =============================================================================
//
// WHAT IT DOES: Scores how bad it is to insert a new entry into a subtree
//
// REQUIREMENT: Return a non-negative float
// REQUIREMENT: Return 0.0 if the new entry fits inside the existing box
// REQUIREMENT: Return area_expansion otherwise
//
// BEHAVIOR:
//   existing box = BBox(0,4,0,4)  area = 16
//   new entry = point (2,2)       already inside → penalty = 0.0
//
//   existing box = BBox(0,4,0,4)  area = 16
//   new entry = point (6,6)       new box = BBox(0,6,0,6) area = 36 → penalty = 20.0
//
//   existing box = BBox(0,4,0,4)
//   new entry = point (5,2)       new box = BBox(0,5,0,4) area = 20 → penalty = 4.0
//
// WHY:
//   During INSERT, GiST walks the tree choosing which subtree to descend into.
//   It calculates PENALTY for each child and picks the one with LOWEST penalty.
//   This keeps bounding boxes tight, which makes future searches faster.
//
//   Bad penalty implementation → fat bounding boxes → slow searches
//   Good penalty implementation → tight bounding boxes → fast searches
//
// =============================================================================
#[pg_guard]
pub unsafe extern "C-unwind" fn vector2d_gist_penalty(
    fcinfo: pg_sys::FunctionCallInfo,
) -> pg_sys::Datum {
    // STEPS:
    //   1. Extract origentry (existing subtree's bounding box) from arg 0
    //   2. Extract newentry (new point being inserted) from arg 1
    //   3. Extract result float pointer from arg 2:
    //      let result = pg_sys::PG_GETARG_POINTER(fcinfo, 2) as *mut f32;
    //
    //   4. Convert newentry to BoundingBox using from_point()
    //   5. Calculate expansion_cost(orig_bbox, new_bbox)
    //   6. Write to result pointer:
    //      *result = cost as f32;
    //
    //   7. Return result pointer as Datum (Postgres convention for penalty):
    //      pg_sys::Datum::from(result as usize)
   let origentry =  (*(*fcinfo).args.as_ptr()).value.cast_mut_ptr::<pg_sys::GISTENTRY>(); //pg_sys::PG_GETARG_POINTER(fcinfo, 0) as *mut pg_sys::GISTENTRY;
   let newentry  =  (*(*fcinfo).args.as_ptr().add(1)).value.cast_mut_ptr::<pg_sys::GISTENTRY>(); //pg_sys::PG_GETARG_POINTER(fcinfo, 1) as *mut pg_sys::GISTENTRY;
   let result = (*(*fcinfo).args.as_ptr().add(2)).value.cast_mut_ptr::<f32>(); //pg_sys::PG_GETARG_POINTER(fcinfo, 2) as *mut f32;

   let orig_bbox = (*origentry).key.cast_mut_ptr::<BoundingBox>();// pg_sys::DatumGetPointer((*origentry).key) as *mut BoundingBox;
   let new_point = (*newentry).key.cast_mut_ptr::<Vector2D>(); // pg_sys::DatumGetPointer((*newentry).key) as *mut Vector2D;
   let new_bbox  = BoundingBox::from_point(&*new_point);

    *result = (*orig_bbox).expansion_cost(&new_bbox) as f32;

    pg_sys::Datum::from(result as usize)
}

// =============================================================================
// GIST SUPPORT FUNCTION 4: PICKSPLIT
// =============================================================================
//
// WHAT IT DOES: Splits an overfull node into two groups
//
// REQUIREMENT: Every entry must go to exactly one group (left or right)
// REQUIREMENT: Both groups must be non-empty
// REQUIREMENT: Should minimize overlap between the two resulting bounding boxes
//
// BEHAVIOR (naive linear split — good enough to start):
//
//   Given entries: (1,1), (2,3), (5,1), (4,4), (8,2), (7,6)
//
//   Step 1: Find seeds
//     leftmost x = (1,1) → seed for left group
//     rightmost x = (8,2) → seed for right group
//
//   Step 2: Assign each remaining entry to the group whose box expands less
//     (2,3): penalty(left+this) vs penalty(right+this) → goes to left
//     (5,1): penalty(left+this) vs penalty(right+this) → goes to right
//     (4,4): penalty(left+this) vs penalty(right+this) → goes to left
//     (7,6): penalty(left+this) vs penalty(right+this) → goes to right
//
//   Result:
//     left group:  (1,1), (2,3), (4,4) → BBox(1,4,1,4)
//     right group: (8,2), (5,1), (7,6) → BBox(5,8,1,6)
//
// WHY:
//   GiST pages have a fixed size. When a page fills up, it must split.
//   A bad split creates overlapping bounding boxes → future searches visit both
//   subtrees instead of pruning one → index becomes slow.
//   A good split minimizes overlap between the two new boxes.
//
// GUTTMAN'S ALGORITHM (what gistproc.c uses, more complex but better):
//   Project all entries onto X axis, find split that minimizes overlap on X
//   Project all entries onto Y axis, find split that minimizes overlap on Y
//   Pick whichever axis gives less overlap
//
// FOR NOW: Use the naive linear split (seed by min/max x).
// We'll improve it in Week 4.
//
// =============================================================================

#[pg_guard]
pub unsafe extern "C-unwind" fn vector2d_gist_picksplit(
    fcinfo: pg_sys::FunctionCallInfo,
) -> pg_sys::Datum {
    let entryvec = (*(*fcinfo).args.as_ptr()).value.cast_mut_ptr::<pg_sys::GistEntryVector>();// pg_sys::PG_GETARG_POINTER(fcinfo, 0) as *mut ;
    let v = (*(*fcinfo).args.as_ptr().add(1)).value.cast_mut_ptr::<pg_sys::GIST_SPLITVEC>(); //pg_sys::PG_GETARG_POINTER(fcinfo, 1) as *mut pg_sys::GIST_SPLITVEC;
    let n = (*entryvec).n as usize;

    // extract all points with their indices
    let mut points: Vec<(Vector2D, usize)> = Vec::new();
    for i in 0..n {
        let key = (*(*entryvec).vector.as_ptr().add(i)).key.cast_mut_ptr::<Vector2D>(); //pg_sys::DatumGetPointer((*entryvec).vector[i].key) as *mut Vector2D;
        points.push((*key, i));
    }

    // split into two groups
    let (left_indices, right_indices) = picksplit_logic(&points);

    // calculate left bounding box
    let mut left_bbox = BoundingBox::from_point(&points[left_indices[0]].0);
    for &idx in &left_indices[1..] {
        left_bbox = left_bbox.expand(&BoundingBox::from_point(&points[idx].0));
    }

    // calculate right bounding box
    let mut right_bbox = BoundingBox::from_point(&points[right_indices[0]].0);
    for &idx in &right_indices[1..] {
        right_bbox = right_bbox.expand(&BoundingBox::from_point(&points[idx].0));
    }

    // fill left array
    (*v).spl_left = pg_sys::palloc(
        left_indices.len() * std::mem::size_of::<pg_sys::OffsetNumber>()
    ) as *mut pg_sys::OffsetNumber;
    for (i, &idx) in left_indices.iter().enumerate() {
        *(*v).spl_left.add(i) = idx as pg_sys::OffsetNumber;
    }
    (*v).spl_nleft = left_indices.len() as i32;

    // fill right array
    (*v).spl_right = pg_sys::palloc(
        right_indices.len() * std::mem::size_of::<pg_sys::OffsetNumber>()
    ) as *mut pg_sys::OffsetNumber;
    for (i, &idx) in right_indices.iter().enumerate() {
        *(*v).spl_right.add(i) = idx as pg_sys::OffsetNumber;
    }
    (*v).spl_nright = right_indices.len() as i32;

    // set bounding boxes
    (*v).spl_ldatum = bbox_to_datum(left_bbox);
    (*v).spl_rdatum = bbox_to_datum(right_bbox);

    pg_sys::Datum::from(v as usize)
}

fn l1_dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn picksplit_logic(points: &[(Vector2D, usize)]) -> (Vec<usize>, Vec<usize>) {
    // find range on each axis
    let mut min_x = points[0].0.x;
    let mut max_x = points[0].0.x;
    let mut min_y = points[0].0.y;
    let mut max_y = points[0].0.y;

    let mut min_seed_x = points[0].0;
    let mut max_seed_x = points[0].0;
    let mut min_seed_y = points[0].0;
    let mut max_seed_y = points[0].0;

    for (p, _) in &points[1..] {
        if p.x < min_x { min_x = p.x; min_seed_x = *p; }
        if p.x > max_x { max_x = p.x; max_seed_x = *p; }
        if p.y < min_y { min_y = p.y; min_seed_y = *p; }
        if p.y > max_y { max_y = p.y; max_seed_y = *p; }
    }

    // pick axis with more spread
    let (min_seed, max_seed) = if max_x - min_x >= max_y - min_y {
        (min_seed_x, max_seed_x)
    } else {
        (min_seed_y, max_seed_y)
    };

    // assign each point to nearest seed
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    for (p, idx) in points {
        let d_min = l1_dist(min_seed.x, min_seed.y, p.x, p.y);
        let d_max = l1_dist(max_seed.x, max_seed.y, p.x, p.y);
        if d_min <= d_max {
            left.push(*idx);
        } else {
            right.push(*idx);
        }
    }

    if left.is_empty(){
        left.push(right.pop().unwrap())
    }
    if right.is_empty(){
        right.push(left.pop().unwrap())
    }

    (left, right)
}
// =============================================================================
// SQL TO CREATE THE OPERATOR CLASS
// (run manually in psql after cargo pgrx run)
// =============================================================================
//
// First define the query type (a circle for range searches):
//
// CREATE TYPE circle_query AS (center_x float8, center_y float8, radius float8);
//
// Define the containment operator:
//
// CREATE FUNCTION vector2d_in_circle(vector2d, circle_query) RETURNS bool ...
//
// CREATE OPERATOR <@ (
//     LEFTARG = vector2d,
//     RIGHTARG = circle_query,
//     FUNCTION = vector2d_in_circle
// );
//
// Create the GiST operator class:
//
// CREATE OPERATOR CLASS vector2d_gist_ops
// DEFAULT FOR TYPE vector2d USING gist AS
//     OPERATOR 1 <@ (vector2d, circle_query),
//     FUNCTION 1 vector2d_gist_consistent(internal, circle_query, smallint, oid, internal),
//     FUNCTION 2 vector2d_gist_union(internal, internal),
//     FUNCTION 3 vector2d_gist_penalty(internal, internal, internal),
//     FUNCTION 4 vector2d_gist_picksplit(internal, internal),
//     STORAGE vector2d;
//
// Test it:
//
// CREATE TABLE locations (id SERIAL, pos vector2d);
// INSERT INTO locations (pos) SELECT ... -- add test data
// CREATE INDEX ON locations USING gist(pos);
// SET enable_seqscan = off;
// EXPLAIN SELECT * FROM locations WHERE pos <@ ROW(0,0,5)::circle_query;
// -- should show: Index Scan using locations_pos_idx
//
// =============================================================================

// =============================================================================
// YOUR 10 TODOs IN ORDER:
//
// WEEK 1 (pure Rust, no unsafe):
//   TODO 1: BoundingBox::from_point     — 3 lines
//   TODO 2: BoundingBox::expand         — 6 lines
//   TODO 3: BoundingBox::area           — 1 line
//   TODO 4: BoundingBox::expansion_cost — 3 lines
//   TODO 5: BoundingBox::overlaps_circle — 5 lines
//   TODO 6: BoundingBox::contains_point — 1 line
//
// WEEK 2 (unsafe Postgres internals):
//   TODO 7: consistent  — prune or go deeper
//   TODO 8: union       — build internal node key
//   TODO 9: penalty     — score insertion cost
//   TODO 10: picksplit  — split full node
//
// Do TODOs 1-6 first. They are pure Rust math.
// No unsafe. No pg_sys. Just structs and arithmetic.
// Tests for each one before moving to TODO 7.
// =============================================================================