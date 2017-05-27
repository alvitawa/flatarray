
pub trait Point {
    ///Same as coord(..), but without bounds checking (make shure index is not bigger than len() if you use this)
    unsafe fn coord_unchecked(&self, index: usize) -> usize;
    fn coord(&self, index: usize) -> usize;
    fn len(&self) -> usize; //The number of coordinates this point has
    fn iter(&self) -> PointCoordsIterator;
}

mod d2 {

}

pub mod d {

    use std::cmp::PartialEq;

    #[derive(Debug)]
    pub struct Point {
        coords: Box<[usize]>
    }

    impl Point {
        pub fn new(coords: Box<[usize]>) -> Point {
            Point {coords}
        }
    }

    impl super::Point for Point {
        unsafe fn coord_unchecked(&self, index: usize) -> usize {
            *self.coords.get_unchecked(index)
        }
        fn coord(&self, index: usize) -> usize {
            self.coords[index]
        }
        fn len(&self) -> usize {
            self.coords.len()
        }
        fn iter(&self) -> super::PointCoordsIterator {
            super::PointCoordsIterator::new(self)
        }
    }

    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            self.coords == other.coords
        }
    }
}

pub struct PointCoordsIterator<'a> {
    point: &'a Point,
    front: usize, // Holds an index
    end: usize // Holds a limit
}

impl<'a> PointCoordsIterator<'a> {
    fn new(point: &'a Point) -> PointCoordsIterator {
        PointCoordsIterator{point: point, front: 0, end: point.len()}
    }
}

impl<'a> Iterator for PointCoordsIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let v: Option<Self::Item>;
        if self.front < self.end {
            unsafe {
                v = Some(self.point.coord_unchecked(self.front));
            }
            self.front += 1;
        } else { 
            v = None; 
        };
        return v
    }
}

impl<'a> DoubleEndedIterator for PointCoordsIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end > self.front {
            self.end -= 1;
            unsafe {
                Some(self.point.coord_unchecked(self.end))
            }
        } else { None }
    }
}