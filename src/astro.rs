//! Wrapper for astro_float::BigFloatNumber

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};
use astro_float::BigFloatNumber;
use astro_float::Exponent;
use astro_float::RoundingMode;
use astro_float::Radix;

pub struct AstroFloat(astro_float::BigFloatNumber);


impl AstroFloat {

    pub fn random_normal(exp_shift: Exponent, exp_range: Exponent) -> Self {
        AstroFloat(BigFloatNumber::random_normal(132, exp_shift, exp_shift + exp_range).unwrap())
    }

    pub fn new(f: BigFloatNumber) -> Self {
        AstroFloat(f)
    }

    pub fn inner(&self) -> &BigFloatNumber {
        &self.0
    }
}


impl Display for AstroFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(Radix::Dec, RoundingMode::ToEven).unwrap())
    }
}

impl Clone for AstroFloat {
    fn clone(&self) -> Self {
        Self(self.0.clone().unwrap())
    }
}

impl Add<Self> for AstroFloat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        AstroFloat(self.0.add(&rhs.0, RoundingMode::ToEven).unwrap())
    }
}

impl<'a> Add<&'a Self> for AstroFloat {
    type Output = Self;

    fn add(self, rhs: &'a Self) -> Self::Output {
        AstroFloat(self.0.add(&rhs.0, RoundingMode::ToEven).unwrap())
    }
}

impl Sub<Self> for AstroFloat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        AstroFloat(self.0.sub(&rhs.0, RoundingMode::ToEven).unwrap())
    }
}

impl<'a> Sub<&'a Self> for AstroFloat {
    type Output = Self;

    fn sub(self, rhs: &'a Self) -> Self::Output {
        AstroFloat(self.0.sub(&rhs.0, RoundingMode::ToEven).unwrap())
    }
}

impl Mul<Self> for AstroFloat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        AstroFloat(self.0.mul(&rhs.0, RoundingMode::ToEven).unwrap())
    }
}

impl<'a> Mul<&'a Self> for AstroFloat {
    type Output = Self;

    fn mul(self, rhs: &'a Self) -> Self::Output {
        AstroFloat(self.0.mul(&rhs.0, RoundingMode::ToEven).unwrap())
    }
}

impl Div<Self> for AstroFloat {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        AstroFloat(self.0.div(&rhs.0, RoundingMode::ToEven).unwrap())
    }
}

impl<'a> Div<&'a Self> for AstroFloat {
    type Output = Self;

    fn div(self, rhs: &'a Self) -> Self::Output {
        AstroFloat(self.0.div(&rhs.0, RoundingMode::ToEven).unwrap())
    }
}

