pub struct Interval {
    pub begin: i32,
    pub end: i32
}

impl Interval {
    pub fn overlap(&self, other: Interval) -> bool {
        if self.begin >= other.begin && self.begin <= other.end {
            return true
        }
        if self.end >= other.begin && self.end <= other.end {
            return true
        }
        if self.begin <= other.begin && self.end >= other.end {
            return true
        }
        false
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // i1: [10, 20], i2: [15, 23]
    #[test]
    fn test_overlap1() {
        let i1 = Interval { begin: 10, end: 20 };
        let i2 = Interval { begin: 15, end: 23 };
        assert_eq!(i1.overlap(i2), true);
    }

    // i1: [10, 20], i2: [5, 23]
    #[test]
    fn test_overlap2() {
        let i1 = Interval { begin: 10, end: 20 };
        let i2 = Interval { begin: 5, end: 23 };
        assert_eq!(i1.overlap(i2), true);
    }

    // i1: [5, 20], i2: [15, 23]
    #[test]
    fn test_overlap3() {
        let i1 = Interval { begin: 5, end: 20 };
        let i2 = Interval { begin: 15, end: 23 };
        assert_eq!(i1.overlap(i2), true);
    }

    // i1: [15, 20], i2: [5, 23]
    #[test]
    fn test_overlap4() {
        let i1 = Interval { begin: 15, end: 20 };
        let i2 = Interval { begin: 5, end: 23 };
        assert_eq!(i1.overlap(i2), true);
    }
}