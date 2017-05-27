
use std::ops::Deref;

use pib::Index;
use pib::PointIndexBinding;
use pib::LimitedReferenceFrame;

use pnt::Point;

use itertools::Itertools as Itr;

// trait TryPush<T> {
//     fn try_push(&self, value: T) -> bool;
// }

// impl<T> TryPush<T> for Vec<T> {
//     fn try_push(&self, value: T) -> bool {
//         if self.len() == self.capacity() {
//             return false;
//         }
//         unsafe {
//             let end = self.as_mut_ptr().offset(self.len() as isize);
//             ptr::write(end, value);
//             self.len() += 1;
//         }
//         return true;
//     }
// }

#[derive(Debug)]
pub struct FlatArray<E, R: PointIndexBinding> {
    lrf: R,
    data: Box<[E]>,
}

impl<E, R: LimitedReferenceFrame> FlatArray<E, R> {
    pub fn by_boxed_slice(lrf: R, bslice: Box<[E]>) -> FlatArray<E, R> {
        FlatArray {
            lrf: lrf,
            data: bslice
        }
    }
    pub fn by_vec(lrf: R, vec: Vec<E>) -> FlatArray<E, R> {
        Self::by_boxed_slice(lrf, vec.into_boxed_slice())
    }
    pub fn by_value(lrf: R, val: E) -> FlatArray<E, R> where E: Copy {
        let mut vec = Vec::with_capacity(lrf.ilimit());
        while vec.len() < vec.capacity() {
            //TODO: vec.push also does a capacity check --> only do what's needed
            vec.push(val);
        }
        Self::by_vec(lrf, vec)
    }
    // pub fn by_point<F>(lrf: R, initializer: F) -> FlatArray<E, R> where F: Fn(R::Point) -> E {
    //     let mut vec = Vec::with_capacity(lrf.ilimit());
    //     let mut ats = Vec::with_capacity(lrf.dim());
    //     while ats.len() < ats.capacity() { ats.push(0); }


    //     ()
    // }
    pub fn by_index<F>(lrf: R, initializer: F) -> FlatArray<E, R> where F: Fn(Index) -> E {
        let mut vec = Vec::with_capacity(lrf.ilimit());
        while vec.len() < vec.capacity() {
            //TODO: vec.push also does a capacity check --> only do what's needed
            let i: usize = vec.len();
            vec.push(initializer(i));
        }
        Self::by_vec(lrf, vec)
    }
    pub fn ix(&self, point: &Point) -> Index {
        self.lrf.index(point)
    }
    pub fn pnt(&self, index: Index) -> R::Point {
        self.lrf.point(index)
    }
    pub fn at(&mut self, point: &Point) -> &mut E {
        &mut self.data[self.ix(point)]
    }
}