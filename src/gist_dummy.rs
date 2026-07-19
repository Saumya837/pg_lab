use pgrx::prelude::*;
use pgrx::pg_sys;
use crate::complex_type;

// ============================================================
// BOUNDING BOX
// This is the "key" stored in GiST internal nodes.
// It represents the smallest box that covers all Complex
// numbers in a subtree.
//
// For Complex numbers (re=x, im=y):
//   min_re = leftmost real part
//   max_re = rightmost real part
//   min_im = lowest imaginary part
//   max_im = highest imaginary part
//
// Example: points (1+2i), (3+4i), (5+1i)
//   BoundingBox { min_re: 1.0, max_re: 5.0, min_im: 1.0, max_im: 4.0 }
// ============================================================
#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct BoundingBox {
    min_re: f64,
    max_re: f64,
    min_im: f64,
    max_im: f64,
}

impl BoundingBox {
    // Create a bounding box from a single Complex point
    fn from_complex(c: &Complex) -> Self {
        let bb = BoundingBox{
            min_re: c.re,
            min_im: c.im,
            max_re: c.re,
            max_im: c.im,
        };
       bb
    }

    // Expand this bounding box to also contain another box
    // Take the min of mins and max of maxes
    // This is what UNION uses
    fn expand(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox{
            min_re: self.min_re.min(other.min_re),
            max_re: self.max_re.max(other.max_re),
            min_im: self.min_im.min(other.min_im),
            max_im: self.max_im.max(other.max_im)
        }
    }

    // Check if this box overlaps with a query circle
    // Query: find all complex numbers within radius r of center point
    // A box overlaps a circle if the closest point on the box to the
    // circle center is within radius r
    fn overlaps_circle(&self, center_re: f64, center_im: f64, radius: f64) -> bool {
        let closest_re = center_re.clamp(self.min_re, self.max_re);
        let closest_im = center_im.clamp(self.min_im,self.max_im);

        let d_re = (closest_re - center_re);
        let d_im = (closest_im - center_im);

        (d_re * d_re + d_im * d_im) <= (radius * radius)
    }

    // Calculate how much area is added by expanding to contain a new box
    // area_after_expansion - current_area
    // Used by PENALTY to decide which subtree to insert into
    fn expansion_cost(&self, other: &BoundingBox) -> f64 {
        let expansion = self.expand(other);
        let expanded= self.expand(other);
        let expanded_area = (expanded.max_re - expanded.min_re) * (expanded.max_im - expanded.min_im);
        expanded_area - current_area
    }
}

// ============================================================
// GIST SUPPORT FUNCTION 1: CONSISTENT
//
// Question: "Does this subtree possibly contain what I'm looking for?"
//
// Arguments:
//   key    = the BoundingBox stored in this GiST node
//   query  = the search query (e.g. center point + radius)
//   level  = 0 means leaf node, > 0 means internal node
//
// Return:
//   true  = YES, go deeper into this subtree
//   false = NO, prune this entire subtree
//
// The logic:
//   At a LEAF node:    check if the actual Complex point is inside the query
//   At INTERNAL node:  check if the BoundingBox overlaps the query
//   (Internal nodes use bounding boxes, leaf nodes have actual values)
// ============================================================
#[pg_extern(immutable, strict, parallel_safe)]
unsafe fn complex_gist_consistent(
    fcinfo: pg_sys::FunctionCallInfo,
) -> pg_sys::Datum {
    // Implement consistent
    // Steps:
    //   1. Extract the GiST entry from fcinfo using GistEntryGetKey
    //   2. Extract the query (center_re, center_im, radius) from fcinfo
    //   3. Check if this is a leaf node (GIST_LEAF macro)
    //   4. If leaf: check if the actual Complex point is inside the query circle
    //   5. If internal: check if the BoundingBox overlaps the query circle
    //   6. Return true/false as Datum
}

// ============================================================
// GIST SUPPORT FUNCTION 2: UNION
//
// Question: "What bounding box covers ALL entries in this node?"
//
// Arguments:
//   entryvec = array of GiST entries (BoundingBoxes or Complex points)
//
// Return:
//   A single BoundingBox that covers all of them
//
// The logic:
//   Start with the first entry's box
//   Expand it to cover each subsequent entry
//   Return the final expanded box
// ============================================================
#[pg_extern(immutable, strict, parallel_safe)]
unsafe fn complex_gist_union(
    fcinfo: pg_sys::FunctionCallInfo,
) -> pg_sys::Datum {
    // TODO 6: Implement union
    // Steps:
    //   1. Extract the GistEntryVector from fcinfo
    //   2. Get the first entry, convert to BoundingBox
    //   3. Loop through remaining entries, expand the box each time
    //   4. palloc a BoundingBox, copy result into it, return as Datum
    todo!("implement union")
}

// ============================================================
// GIST SUPPORT FUNCTION 3: PENALTY
//
// Question: "How bad is it to insert this new entry into this subtree?"
//
// Arguments:
//   origentry = the existing BoundingBox of the subtree
//   newentry  = the new entry being inserted
//
// Return:
//   A float representing the cost (area expansion)
//   Lower is better — GiST will insert into the subtree with lowest penalty
//
// The logic:
//   Calculate how much the BoundingBox would grow if we added the new entry
//   new_area - old_area = penalty
// ============================================================
#[pg_extern(immutable, strict, parallel_safe)]
unsafe fn complex_gist_penalty(
    fcinfo: pg_sys::FunctionCallInfo,
) -> pg_sys::Datum {
    // TODO 7: Implement penalty
    // Steps:
    //   1. Extract origentry BoundingBox from fcinfo
    //   2. Extract newentry (Complex point or BoundingBox) from fcinfo
    //   3. Calculate expansion_cost
    //   4. Write result into the penalty float pointer in fcinfo
    //   5. Return the penalty pointer as Datum
    todo!("implement penalty")
}

// ============================================================
// GIST SUPPORT FUNCTION 4: PICKSPLIT
//
// Question: "This node is full — split it into two groups"
//
// Arguments:
//   entryvec = all the entries that need to be split
//   v        = output struct — fill v->spl_left and v->spl_right
//
// Return:
//   Fill the GIST_SPLITVEC with two groups
//
// The logic (naive linear split):
//   Find the entry with the smallest re (leftmost)
//   Find the entry with the largest re (rightmost)
//   Put leftmost in group 1, rightmost in group 2
//   For remaining entries: assign to whichever group's box expands less
// ============================================================
#[pg_extern(immutable, strict, parallel_safe)]
unsafe fn complex_gist_picksplit(
    fcinfo: pg_sys::FunctionCallInfo,
) -> pg_sys::Datum {
    // TODO 8: Implement picksplit
    // Steps:
    //   1. Extract GistEntryVector from fcinfo
    //   2. Find entry with min re → seed for left group
    //   3. Find entry with max re → seed for right group
    //   4. For each remaining entry:
    //      a. Calculate penalty of adding to left group
    //      b. Calculate penalty of adding to right group
    //      c. Add to whichever has lower penalty
    //   5. Fill v->spl_left, v->spl_right, v->spl_ldatum, v->spl_rdatum
    todo!("implement picksplit")
}

// ============================================================
// SQL TO RUN AFTER COMPILING
// (run manually in psql since pgrx doesn't auto-generate this)
// ============================================================

// CREATE OPERATOR CLASS complex_gist_ops
// DEFAULT FOR TYPE complex USING gist AS
//     OPERATOR 1 <@ (complex, complex_circle),   -- contained in circle
//     FUNCTION 1 complex_gist_consistent(internal, complex_circle, smallint, oid, internal),
//     FUNCTION 2 complex_gist_union(internal, internal),
//     FUNCTION 3 complex_gist_penalty(internal, internal, internal),
//     FUNCTION 4 complex_gist_picksplit(internal, internal),
//     STORAGE complex;
//
// Then test:
// CREATE INDEX ON signals USING gist(value);
// SELECT * FROM signals WHERE value <@ complex_circle('(0,0)', 5.0);