use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

use ordered_float::OrderedFloat;

use crate::{EvalexprError, EvalexprResult, Value};

use super::{EvalexprFloat, EvalexprInt, EvalexprNumericTypes};

/// See [`EvalexprNumericTypes`].
///
/// This empty struct uses [`i64`] as its integer type and [`f64`] as its float type.
#[cfg_attr(
    feature = "serde",
    derive(Default, serde::Serialize, serde::Deserialize)
)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DefaultNumericTypes;

impl EvalexprNumericTypes for DefaultNumericTypes {
    type Int = i64;
    type Float = OrderedFloat<f64>;

    fn int_as_float(int: &Self::Int) -> Self::Float {
        OrderedFloat(*int as f64)
    }

    fn float_as_int(float: &Self::Float) -> Self::Int {
        float.0 as Self::Int
    }
}

impl<NumericTypes: EvalexprNumericTypes<Int = Self>> EvalexprInt<NumericTypes> for i64 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;

    fn from_usize(int: usize) -> EvalexprResult<Self, NumericTypes> {
        int.try_into()
            .map_err(|_| EvalexprError::IntFromUsize { usize_int: int })
    }

    fn into_usize(&self) -> EvalexprResult<usize, NumericTypes> {
        if *self >= 0 {
            (*self as u64)
                .try_into()
                .map_err(|_| EvalexprError::IntIntoUsize { int: *self })
        } else {
            Err(EvalexprError::IntIntoUsize { int: *self })
        }
    }

    fn from_hex_str(literal: &str) -> Result<Self, ()> {
        Self::from_str_radix(literal, 16).map_err(|_| ())
    }

    fn from_binary_str(literal: &str) -> Result<Self, ()> {
        Self::from_str_radix(literal, 2).map_err(|_| ())
    }

    fn from_octal_str(literal: &str) -> Result<Self, ()> {
        Self::from_str_radix(literal, 8).map_err(|_| ())
    }

    fn checked_add(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes> {
        let result = (*self).checked_add(*rhs);
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(EvalexprError::addition_error(
                Value::<NumericTypes>::from_int(*self),
                Value::<NumericTypes>::from_int(*rhs),
            ))
        }
    }

    fn checked_sub(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes> {
        let result = (*self).checked_sub(*rhs);
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(EvalexprError::subtraction_error(
                Value::<NumericTypes>::from_int(*self),
                Value::<NumericTypes>::from_int(*rhs),
            ))
        }
    }

    fn checked_neg(&self) -> EvalexprResult<Self, NumericTypes> {
        let result = (*self).checked_neg();
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(EvalexprError::negation_error(
                Value::<NumericTypes>::from_int(*self),
            ))
        }
    }

    fn checked_mul(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes> {
        let result = (*self).checked_mul(*rhs);
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(EvalexprError::multiplication_error(
                Value::<NumericTypes>::from_int(*self),
                Value::<NumericTypes>::from_int(*rhs),
            ))
        }
    }

    fn checked_div(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes> {
        let result = (*self).checked_div(*rhs);
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(EvalexprError::division_error(
                Value::<NumericTypes>::from_int(*self),
                Value::<NumericTypes>::from_int(*rhs),
            ))
        }
    }

    fn checked_rem(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes> {
        let result = (*self).checked_rem(*rhs);
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(EvalexprError::modulation_error(
                Value::<NumericTypes>::from_int(*self),
                Value::<NumericTypes>::from_int(*rhs),
            ))
        }
    }

    fn abs(&self) -> EvalexprResult<Self, NumericTypes> {
        Ok((*self).abs())
    }

    fn bitand(&self, rhs: &Self) -> Self {
        BitAnd::bitand(*self, *rhs)
    }

    fn bitor(&self, rhs: &Self) -> Self {
        BitOr::bitor(*self, *rhs)
    }

    fn bitxor(&self, rhs: &Self) -> Self {
        BitXor::bitxor(*self, *rhs)
    }

    fn bitnot(&self) -> Self {
        Not::not(*self)
    }

    fn bit_shift_left(&self, rhs: &Self) -> Self {
        Shl::shl(*self, *rhs)
    }

    fn bit_shift_right(&self, rhs: &Self) -> Self {
        Shr::shr(*self, *rhs)
    }
}

impl<NumericTypes: EvalexprNumericTypes<Float = Self>> EvalexprFloat<NumericTypes>
    for OrderedFloat<f64>
{
    const MIN: Self = OrderedFloat(f64::NEG_INFINITY);
    const MAX: Self = OrderedFloat(f64::INFINITY);

    fn pow(&self, exponent: Self) -> Self {
        OrderedFloat(self.0.powf(exponent.0))
    }

    fn ln(&self) -> Self {
        OrderedFloat(self.0.ln())
    }

    fn log(&self, base: Self) -> Self {
        OrderedFloat(self.0.log(base.0))
    }

    fn log2(&self) -> Self {
        OrderedFloat(self.0.log2())
    }

    fn log10(&self) -> Self {
        OrderedFloat(self.0.log10())
    }

    fn exp(&self) -> Self {
        OrderedFloat(self.0.exp())
    }

    fn exp2(&self) -> Self {
        OrderedFloat(self.0.exp2())
    }

    fn cos(&self) -> Self {
        OrderedFloat(self.0.cos())
    }

    fn cosh(&self) -> Self {
        OrderedFloat(self.0.cosh())
    }

    fn acos(&self) -> Self {
        OrderedFloat(self.0.acos())
    }

    fn acosh(&self) -> Self {
        OrderedFloat(self.0.acosh())
    }

    fn sin(&self) -> Self {
        OrderedFloat(self.0.sin())
    }

    fn sinh(&self) -> Self {
        OrderedFloat(self.0.sinh())
    }

    fn asin(&self) -> Self {
        OrderedFloat(self.0.asin())
    }

    fn asinh(&self) -> Self {
        OrderedFloat(self.0.asinh())
    }

    fn tan(&self) -> Self {
        OrderedFloat(self.0.tan())
    }

    fn tanh(&self) -> Self {
        OrderedFloat(self.0.tanh())
    }

    fn atan(&self) -> Self {
        OrderedFloat(self.0.atan())
    }

    fn atanh(&self) -> Self {
        OrderedFloat(self.0.atanh())
    }

    fn atan2(&self, x: Self) -> Self {
        OrderedFloat(self.0.atan2(x.0))
    }

    fn sqrt(&self) -> Self {
        OrderedFloat(self.0.sqrt())
    }

    fn cbrt(&self) -> Self {
        OrderedFloat(self.0.cbrt())
    }

    fn hypot(&self, other: Self) -> Self {
        OrderedFloat(self.0.hypot(other.0))
    }

    fn floor(&self) -> Self {
        OrderedFloat(self.0.floor())
    }

    fn round(&self) -> Self {
        OrderedFloat(self.0.round())
    }

    fn ceil(&self) -> Self {
        OrderedFloat(self.0.ceil())
    }

    fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }

    fn is_normal(&self) -> bool {
        self.0.is_normal()
    }

    fn abs(&self) -> Self {
        OrderedFloat(self.0.abs())
    }

    fn min(self, other: Self) -> Self {
        OrderedFloat(self.0.min(other.0))
    }

    fn max(self, other: Self) -> Self {
        OrderedFloat(self.0.max(other.0))
    }

    fn random() -> EvalexprResult<Self, NumericTypes> {
        #[cfg(feature = "rand")]
        let result = Ok(OrderedFloat(rand::random()));

        #[cfg(not(feature = "rand"))]
        let result = Err(EvalexprError::RandNotEnabled);

        result
    }
}
