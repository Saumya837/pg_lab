use pgrx::prelude::*;
use pgrx::pg_sys;


#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}


#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct BoundingBox {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
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
        (expanded.area() - self.area())
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
        let closest_x = center_x.clamp(min_x, max_x);
        let closest_y = center_y.clamp(min_x, max_y);

        dx = center_x - closest_x;
        dy = center_y - closest_y;

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

unsafe fn get_entry_count(entryvec: *mut pg_sys::GistEntryVector) -> i32
{
    ((*entryvec).n as i32)
}

unsafe fn get_point(entryvec: *mut pg_sys::GistEntryVector, i:usize) -> Vector2D{
    let datum = (*entryvec).vector[i].key; //datum pointer
    let point = pg_sys::DatumGetPointer(datum)as *mut Vector2D; //datum pointer -> vector pointer
    (*point) 
}

unsafe fn get_bbox_at(
    entry_vec: *mut pg_sys::GistEntryVector,
    i: usize
) -> BoundingBox {
    // your code here
    let datum = (*entry_vec).vector[i].key;
    let point = pg_sys::DatumGetPointer(datum) as *mut Vector2D;
    BoundingBox::from_point(&*point)
}

unsafe fn bbox_to_datum(bbox: BoundingBox) -> pg_sys::Datum {
    // allocate memory using palloc
    let ptr = pg_sys::palloc(std::mem::size_of::<BoundingBox>()) as *mut BoundingBox;

    // write bbox into it
    *ptr = bbox;

    // return as Datum
    pg_sys::PointerGetDatum(ptr as *mut std::ffi::c_void)
}


fn fill_splitvec(
    v: *mut pg_sys::GIST_SPLITVEC,
    left_indices: &[usize],
    right_indices: &[usize],
    left_bbox: BoundingBox,
    right_bbox: BoundingBox,
) -> i32 {

    (*v).spl_left = pg_sys::palloc(
       left_indices.len() * std::mem::size_of::<pg_sys::OffsetNumber>() 
    ) as *mut pg_sys::OffsetNumber;

    //fill left array
    for (i, idx) in left_indices.iter().enumerate(){
        *(*v).spl_left.add(i) = *idx as pg_sys::OffsetNumber;
    }

    (*v).spl_nleft = left_indices.len() as i32;
    
    // allocate right array
    (*v).spl_right = pg_sys::palloc(
        right_indices.len() * std::mem::size_of::<pg_sys::OffsetNumber>()
    ) as *mut pg_sys::OffsetNumber;

    // fill right array
    for (i, idx) in right_indices.iter().enumerate() {
        *(*v).spl_right.add(i) = *idx as pg_sys::OffsetNumber;
    }

    // set right count
    (*v).spl_nright = right_indices.len() as i32;

    // set bounding boxes
    (*v).spl_ldatum = bbox_to_datum(left_bbox);
    (*v).spl_rdatum = bbox_to_datum(right_bbox);
    
}
