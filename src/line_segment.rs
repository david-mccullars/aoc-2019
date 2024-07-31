use crate::{manhattan_distance, Pos};
use std::cmp::{max, min};
use std::iter::{empty, once};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct LineSegment {
    pub p1: Pos,
    pub p2: Pos,
}

impl LineSegment {
    pub fn vertical(&self) -> bool {
        self.p1.0 == self.p2.0 && self.p1.1 != self.p2.1
    }

    pub fn horizontal(&self) -> bool {
        !self.vertical()
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> isize {
        manhattan_distance(&self.p1, &self.p2)
    }

    pub fn intersect(&self, other: &LineSegment) -> Option<Pos> {
        let mut i = self.intersections(other);
        let intersection = i.next();
        assert!(i.next().is_none(), "More than one intersection");
        intersection
    }

    pub fn intersections(&self, other: &LineSegment) -> Box<dyn Iterator<Item = Pos> + '_> {
        if self.vertical() ^ other.vertical() {
            self.intersections_orthogonal(other)
        } else {
            self.intersections_parallel(other)
        }
    }

    fn intersections_orthogonal(&self, other: &LineSegment) -> Box<dyn Iterator<Item = Pos> + '_> {
        let (h, v) = if self.vertical() {
            (other, self)
        } else {
            (self, other)
        };
        if in_line(h.p1.0, h.p2.0, v.p1.0) && in_line(v.p1.1, v.p2.1, h.p1.1) {
            Box::new(once((v.p1.0, h.p1.1)))
        } else {
            Box::new(empty())
        }
    }

    #[allow(clippy::reversed_empty_ranges)]
    fn intersections_parallel(&self, other: &LineSegment) -> Box<dyn Iterator<Item = Pos> + '_> {
        if self.vertical() && self.p1.0 == other.p1.0 {
            Box::new(overlap(self.p1.1, self.p2.1, other.p1.1, other.p2.1).map(|y| (self.p1.0, y)))
        } else if self.horizontal() && self.p1.1 == other.p1.1 {
            Box::new(overlap(self.p1.0, self.p2.0, other.p1.0, other.p2.0).map(|x| (x, self.p1.1)))
        } else {
            Box::new(empty())
        }
    }
}

#[inline]
fn in_line(x1: isize, x2: isize, x: isize) -> bool {
    (x1 <= x && x <= x2) || (x2 <= x && x <= x1)
}

#[inline]
fn overlap(i1: isize, i2: isize, j1: isize, j2: isize) -> std::ops::RangeInclusive<isize> {
    let k1 = max(min(i1, i2), min(j1, j2));
    let k2 = min(max(i1, i2), max(j1, j2));
    k1..=k2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pos_from;

    fn line(x1: isize, y1: isize, x2: isize, y2: isize) -> LineSegment {
        LineSegment {
            p1: pos_from(x1, y1),
            p2: pos_from(x2, y2),
        }
    }

    #[test]
    fn test_intersect1() {
        let a = line(1, 5, 1, 9);
        let b = line(0, 7, 12, 7);
        assert_eq!(a.intersect(&b), Some((1, 7)));
    }

    #[test]
    fn test_intersect2() {
        let a = line(1, 5, 1, 9);
        let b = line(12, 7, 0, 7);
        assert_eq!(a.intersect(&b), Some((1, 7)));
    }

    #[test]
    fn test_intersect3() {
        let a = line(1, 9, 1, 5);
        let b = line(0, 7, 12, 7);
        assert_eq!(a.intersect(&b), Some((1, 7)));
    }

    #[test]
    fn test_intersect4() {
        let a = line(1, 9, 1, 5);
        let b = line(12, 7, 0, 7);
        assert_eq!(a.intersect(&b), Some((1, 7)));
    }

    #[test]
    fn test_intersect5() {
        let a = line(1, 5, 1, 9);
        let b = line(0, 7, 12, 7);
        assert_eq!(b.intersect(&a), Some((1, 7)));
    }

    #[test]
    fn test_intersect6() {
        let a = line(1, 5, 1, 9);
        let b = line(12, 7, 0, 7);
        assert_eq!(b.intersect(&a), Some((1, 7)));
    }

    #[test]
    fn test_intersect7() {
        let a = line(1, 9, 1, 5);
        let b = line(0, 7, 12, 7);
        assert_eq!(b.intersect(&a), Some((1, 7)));
    }

    #[test]
    fn test_intersect8() {
        let a = line(1, 9, 1, 5);
        let b = line(12, 7, 0, 7);
        assert_eq!(b.intersect(&a), Some((1, 7)));
    }

    #[test]
    fn test_intersect_none1() {
        let a = line(1, 9, 1, 5);
        let b = line(2, 7, 12, 7);
        assert_eq!(a.intersect(&b), None);
    }

    #[test]
    fn test_intersect_none2() {
        let a = line(3, 5, 3, 2);
        let b = line(0, 7, 6, 7);
        assert_eq!(a.intersect(&b), None);
    }

    #[test]
    fn test_intersect_many1() {
        let a = line(3, 5, 3, 2);
        let b = line(3, 7, 3, 3);
        assert_eq!(
            a.intersections(&b).collect::<Vec<_>>(),
            vec![(3, 3), (3, 4), (3, 5)]
        );
    }

    #[test]
    fn test_intersect_many2() {
        let a = line(192, 7, 5, 7);
        let b = line(-331, 7, 6, 7);
        assert_eq!(
            a.intersections(&b).collect::<Vec<_>>(),
            vec![(5, 7), (6, 7)]
        );
    }
}
