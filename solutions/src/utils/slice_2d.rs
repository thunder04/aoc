use std::ops::{Deref, Index};

pub struct Slice2D<'a> {
    slice: &'a [u8],
    row_len: usize,
}

impl<'a> Slice2D<'a> {
    pub const fn new(slice: &'a [u8], row_len: usize) -> Self {
        Self { slice, row_len }
    }
}

impl Index<(usize, usize)> for Slice2D<'_> {
    type Output = u8;

    #[inline(always)]
    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        assert!(row < self.row_len);

        &self.slice[self.row_len * col + row]
    }
}

impl Deref for Slice2D<'_> {
    type Target = [u8];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.slice
    }
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const SLICE: Slice2D = Slice2D::new(ARRAY, ROW_LEN);
    const ROW_LEN: usize = 6;
    #[rustfmt::skip]
    const ARRAY: &[u8] = &[
        // ·     0   1   2   3   4   5
        /* 0 */ 29, 32, 54, 19, 32, 43,
        /* 1 */ 10, 19, 14, 34, 19, 39,
        /* 2 */ 10, 12, 14, 16, 18, 20,
        /* 3 */ 23, 26, 89, 31, 33, 35,
    ];

    #[test]
    fn valid_access() {
        assert_eq!(SLICE[(0, 0)], 29);
        assert_eq!(SLICE[(0, 5)], 43);
        assert_eq!(SLICE[(1, 0)], 10);
        assert_eq!(SLICE[(1, 1)], 19);
        assert_eq!(SLICE[(1, 5)], 39);
        assert_eq!(SLICE[(3, 5)], 35);
    }

    #[test]
    #[should_panic]
    fn invalid_access_1() {
        black_box(SLICE[(0, 6)]);
    }

    #[test]
    #[should_panic]
    fn invalid_access_2() {
        black_box(SLICE[(6, 6)]);
    }

    #[test]
    #[should_panic]
    fn invalid_access_3() {
        black_box(SLICE[(6, 1)]);
    }
}