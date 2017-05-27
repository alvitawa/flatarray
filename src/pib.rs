
use std::fmt;

use pnt::Point;

pub type Index = usize;

pub trait PointIndexBinding : fmt::Debug {
    type Point : Point;

    fn point(&self, index: Index) -> Self::Point;
    //Using &Point instead of &Self::Point because this method shouldn't care
    fn index(&self, point: &Point) -> Index;
    ///Highest value that can be returned by index() (assuming safe usage) plus 1
    fn ilimit(&self) -> usize;
    fn dim(&self) -> usize;
}

/// A PointIndexBinding that stores the limits it is initialized with
pub trait LimitedReferenceFrame : PointIndexBinding {
    type PIB;
    fn limits(&self) -> &[usize];
    fn to_pib(self) -> Self::PIB;
}

pub mod d2 {

}

pub mod d {
    use Index;
    use pnt::d::Point;

    #[derive(Debug)]
    pub struct PointIndexBinding {
        mults: Box<[usize]>,
        point_len: usize
    }

    impl super::PointIndexBinding for PointIndexBinding {
        type Point = Point;
        /*
            The formula used here to calculate the coordinates of the point to be returned is:

            c[i] = ((n % m[i+1]) - (n % m[i])) / m[i]

            Where c is an array to store these coordinates, n the index parameter
            and m an array holding the multipliers.
        */
        fn point(&self, index: Index) -> Self::Point {
            let mut b: usize = 0;
            Self::Point::new((0..(self.point_len)).map(|i| {
                let a = index % self.mults[i+1];
                let r = (a - b) / self.mults[i];
                b = a;
                return r
            }).collect::<Vec<usize>>().into_boxed_slice())

            ////Tested slower, probably because of double bound checking
            // let mut coords: Vec<usize> = Vec::with_capacity(self.mults.len()-1);

            // let mut b = 0; // index % mults[0] is always 0
            // while coords.len() < self.mults.len()-1 {
            //     let a = index % self.mults[coords.len()+1];
            //     let mult = self.mults[coords.len()];
            //     coords.push((a - b) / mult);
            //     b = a;
            // }
            // Point::new(coords.into_boxed_slice())
        }

        fn index(&self, point: &super::Point) -> Index {
            self.mults.iter().zip((*point).iter()).fold(0, |acc, pair| acc + pair.0 * pair.1)
        }

        fn ilimit(&self) -> usize { self.mults[self.mults.len()-1] }
        fn dim(&self) -> usize { self.point_len }
    }

    impl PointIndexBinding {

        pub fn new(limits: &[usize]) -> Self {
            let mut mults = Vec::with_capacity(limits.len()+1);
            mults.push(1);

            let mut mult: usize = mults[0];
            while mults.len() <= limits.len() {
                mult *= limits[mults.len()-1];
                mults.push(mult);
            }

            PointIndexBinding {
                mults: mults.into_boxed_slice(),
                point_len: limits.len()
            }
        }

        pub fn multiplier(&self, dimension: usize) -> usize {
            self.mults[dimension]
        }
    }


    #[derive(Debug)]
    pub struct LimitedReferenceFrame {
        pib: PointIndexBinding,
        limits: Box<[usize]>
    }

    impl super::PointIndexBinding for LimitedReferenceFrame {
        type Point = Point;

        fn point(&self, index: Index) -> Self::Point { self.pib.point(index) }
        fn index(&self, point: &super::Point) -> Index { self.pib.index(point) }
        fn ilimit(&self) -> usize { self.pib.ilimit() }
        fn dim(&self) -> usize { self.pib.dim() }
    }

    impl super::LimitedReferenceFrame for LimitedReferenceFrame {
        type PIB = PointIndexBinding;
        fn limits(&self) -> &[usize] { &self.limits }
        fn to_pib(self) -> Self::PIB {
            self.pib
        }
    }

    impl LimitedReferenceFrame {
        pub fn new(limits: Box<[usize]>) -> Self {
            LimitedReferenceFrame {
                pib: PointIndexBinding::new(&limits),
                limits: limits,
            }
        }
    }
}
