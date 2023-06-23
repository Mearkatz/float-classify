use num::traits::Zero;

pub trait Category {
    /// Type returned from `destructure`.
    /// Should probably be CatFloat
    type D;
    /// Splits an f64 into its Integer and Fractional parts.
    ///
    /// # Examples:
    /// ```rust
    /// # use floating_cat::*;
    /// let n: f64 = 1.5;
    /// assert_eq!(n.category(), CatFloat::IntegerAndFractionalPart(1.0, 0.5));
    /// ```
    fn category(&self) -> Self::D;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CatFloat {
    /// For `f64`s like `1.0`, `-100.0`, with no fractional part.
    /// Being integer-like means this can usually be casted as an integer without issue.
    /// Note: `f64::MAX > u128::MAX`
    IntegerLike(f64),

    /// For `f64`s like `0.5` or `-0.002`, where there's no integer part.
    /// Casting this as an integer wouldn't be recommended, since you'd lose information.
    FractionLike(f64),

    /// The Integer and Fractional parts of an f64, in that order.
    IntegerAndFractionalPart(f64, f64),

    /// The Float was NaN
    Nan,

    /// The Float was Infinity
    Infinity,
}

impl CatFloat {
    /// Returns `true` if the Classified float is [`IntegerLike`].
    ///
    /// [`IntegerLike`]: CatFloat::IntegerLike    
    pub fn is_integer_like(&self) -> bool {
        matches!(self, Self::IntegerLike(..))
    }

    /// Returns `true` if the Classified float is [`FractionLike`].
    ///
    /// [`FractionLike`]: CatFloat::FractionLike    
    pub fn is_fraction_like(&self) -> bool {
        matches!(self, Self::FractionLike(..))
    }

    /// Returns `true` if the Classified float is [`IntegerAndFractionalPart`].
    ///
    /// [`IntegerAndFractionalPart`]: CatFloat::IntegerAndFractionalPart    
    pub fn is_integer_and_fractional_part(&self) -> bool {
        matches!(self, Self::IntegerAndFractionalPart(..))
    }

    /// Returns `true` if the Classified float is [`Infinity`].
    ///
    /// [`Infinity`]: CatFloat::Infinity    
    pub fn is_infinity(&self) -> bool {
        matches!(self, Self::Infinity)
    }

    /// Returns `true` if the Classified float is [`Nan`].
    ///
    /// [`Nan`]: CatFloat::Nan    
    pub fn is_nan(&self) -> bool {
        matches!(self, Self::Nan)
    }
}

impl Category for f64 {
    type D = CatFloat;
    fn category(&self) -> Self::D {
        if self.is_infinite() {
            return CatFloat::Infinity;
        }
        if self.is_nan() {
            return CatFloat::Nan;
        }

        let int_part: f64 = self.trunc();
        let fract_part: f64 = self.fract();

        if fract_part.is_zero() {
            CatFloat::IntegerLike(int_part)
        } else if int_part.is_zero() {
            CatFloat::FractionLike(fract_part)
        } else {
            CatFloat::IntegerAndFractionalPart(int_part, fract_part)
        }
    }
}

#[test]
fn trait_works() {
    use crate::*;

    let f: f64 = 1.5;
    assert_eq!(f.category(), CatFloat::IntegerAndFractionalPart(1.0, 0.5));

    let f: f64 = 1.0;
    assert_eq!(f.category(), CatFloat::IntegerLike(1.0));

    let f: f64 = 0.2;
    assert_eq!(f.category(), CatFloat::FractionLike(0.2));

    let f: f64 = f64::INFINITY;
    assert_eq!(f.category(), CatFloat::Infinity,);

    let f: f64 = f64::NAN;
    assert_eq!(f.category(), CatFloat::Nan);
}
