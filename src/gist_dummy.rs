// use pgrx::prelude::*;

// #[derive(Copy, Clone, Debug, PostgresType, Serialize, Deserilaize)]
// pub struct IntSpan{
//     pub lo: i32,
//     pub hi: i32,
// }

// impl IntSpan{
//     fn overlaps(&self, other: &IntSpan) -> bool {
//         self.lo <= other.hi && other.lo <= self.hi
//     }
//     fn contains(&self, other: &IntSpan) -> bool {
//         self.lo <= other.hi && self.hi >= other.hi
//     }

// }