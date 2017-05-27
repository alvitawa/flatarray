#![feature(box_syntax)]
#![feature(generic_param_attrs)]
#![feature(dropck_eyepatch)]
#![feature(alloc)]
#![feature(test)]
#![feature(use_extern_macros)]

#[macro_use] extern crate itertools;
extern crate test;

pub use fa::FlatArray;

pub use pib::Index;
pub use pib::PointIndexBinding;
pub use pib::LimitedReferenceFrame;

pub use pnt::Point;

pub mod d {
    pub use pib::d::LimitedReferenceFrame;
    pub use pib::d::PointIndexBinding;
    pub use pnt::d::Point;
}

mod fa;
mod pnt;
mod pib;

#[cfg(test)]
mod tests {

    use test;
    use itertools;
    use itertools::Itertools;
    use PointIndexBinding;
    use d;

    #[bench]
    fn vecinits(b: &mut test::Bencher) {
        let pib = d::PointIndexBinding::new(&[2,2,2]);
        b.iter(|| {
            assert!(pib.point(5) == d::Point::new(box [1,0,1]))
            // println!("{:?}", )
        })
    }

    // #[bench]
    // fn vecinits_alt(b: &mut test::Bencher) {
    //     let pib = d::PointIndexBinding::new(&[2,2,2]);
    //     b.iter(|| {
    //         assert!(pib.point_alt(5) == d::Point::new(box [1,0,1]))
    //         // println!("{:?}", )
    //     })
    // }

    use FlatArray;
    use Index;

    #[test]
    fn fatest() {
        let arr: FlatArray<usize, d::LimitedReferenceFrame> 
            = FlatArray::by_index(d::LimitedReferenceFrame::new(box [2,2,2]), |i: Index| -> usize {i*2});
        println!("{:?}", arr);
    }

    // #[test]
    fn ptest() {
        let mut arr: [usize; 4] = [23,3,42,2];
        let barr: Box<[usize; 4]> = box arr;
        let d = d::LimitedReferenceFrame::new(barr);
        println!("{:?}", d);

        arr[2] = 0;

        println!("{:?}", arr);
        println!("{:?}", d);
    }
}
