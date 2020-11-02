fn main() {
    use std::ops::BitXor;
    use std::ops::BitAndAssign;

    #[derive(Debug, PartialEq)]
    struct BooleanVector(Vec<bool>);

    impl BitXor for BooleanVector {
        type Output = Self;

        fn bitxor(self, Self(rhs): Self) -> Self::Output {
            let Self(lhs) = self;
            assert_eq!(lhs.len(), rhs.len());
            Self(
                lhs.iter()
                    .zip(rhs.iter())
                    .map(|(x, y)| (*x || *y) && !(*x && *y))
                    .collect(),
            )
        }
    }

    let bv1 = BooleanVector(vec![true, true, false, false]);
    let bv2 = BooleanVector(vec![true, false, true, false]);
    let expected = BooleanVector(vec![false, true, true, false]);
    assert_eq!(bv1 ^ bv2, expected);

    impl BitAndAssign for BooleanVector {
        // `rhs` is the "right-hand side" of the expression `a &= b`.
        fn bitand_assign(&mut self, rhs: Self) {
            assert_eq!(self.0.len(), rhs.0.len());
            *self = Self(
                self.0
                    .iter()
                    .zip(rhs.0.iter())
                    .map(|(x, y)| *x & *y)
                    .collect()
            );
        }
    }

    let mut bv = BooleanVector(vec![true, true, false, false]);
    bv &= BooleanVector(vec![true, false, true, false]);
    let expected = BooleanVector(vec![true, false, false, false]);
    assert_eq!(bv, expected);
}
