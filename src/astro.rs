//! Wrapper for astro_float::BigFloat

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
    cell::RefCell,
    rc::Rc,
};
use astro_float::{BigFloat, Consts, Sign};
use astro_float::Exponent;
use astro_float::RoundingMode;

pub struct AstroFloat
{
    af: BigFloat,
    pub cc: Rc<RefCell<Consts>>,
}


impl AstroFloat {

    pub fn random_normal(exp_from: Exponent, exp_to: Exponent, cc: Rc<RefCell<Consts>>, sign_positive: bool) -> Self {
        let exp_from = (exp_from as i64 * 3321928095 / 1000000000) as Exponent;
        let exp_to = (exp_to as i64 * 3321928095 / 1000000000) as Exponent;
        let mut af = BigFloat::random_normal(132, exp_from, exp_to);
        if sign_positive {
            af.set_sign(Sign::Pos);
        }
        AstroFloat {
            af,
            cc,
        }
    }

    pub fn new(f: BigFloat, cc: Rc<RefCell<Consts>>) -> Self {
        AstroFloat {
            af: f,
            cc,
        }
    }

    pub fn inner(&self) -> &BigFloat {
        &self.af
    }
}


impl Display for AstroFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.af)
    }
}

impl Clone for AstroFloat {
    fn clone(&self) -> Self {
        Self {
            af: self.af.clone(),
            cc: self.cc.clone(),
        }
    }
}

impl Add<Self> for AstroFloat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        AstroFloat {
            af: self.af.add(&rhs.af, self.af.mantissa_max_bit_len().unwrap_or(1), RoundingMode::ToEven),
            cc: self.cc
        }
    }
}

impl<'a> Add<&'a Self> for AstroFloat {
    type Output = Self;

    fn add(self, rhs: &'a Self) -> Self::Output {
        AstroFloat {
            af: self.af.add(&rhs.af, self.af.mantissa_max_bit_len().unwrap_or(1), RoundingMode::ToEven),
            cc: self.cc
        }
    }
}

impl Sub<Self> for AstroFloat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        AstroFloat {
            af: self.af.sub(&rhs.af, self.af.mantissa_max_bit_len().unwrap_or(1), RoundingMode::ToEven),
            cc: self.cc
        }
    }
}

impl<'a> Sub<&'a Self> for AstroFloat {
    type Output = Self;

    fn sub(self, rhs: &'a Self) -> Self::Output {
        AstroFloat {
            af: self.af.sub(&rhs.af, self.af.mantissa_max_bit_len().unwrap_or(1), RoundingMode::ToEven),
            cc: self.cc
        }
    }
}

impl Mul<Self> for AstroFloat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        AstroFloat {
            af: self.af.mul(&rhs.af, self.af.mantissa_max_bit_len().unwrap_or(1), RoundingMode::ToEven),
            cc: self.cc
        }
    }
}

impl<'a> Mul<&'a Self> for AstroFloat {
    type Output = Self;

    fn mul(self, rhs: &'a Self) -> Self::Output {
        AstroFloat {
            af: self.af.mul(&rhs.af, self.af.mantissa_max_bit_len().unwrap_or(1), RoundingMode::ToEven),
            cc: self.cc
        }
    }
}

impl Div<Self> for AstroFloat {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        AstroFloat {
            af: self.af.div(&rhs.af, self.af.mantissa_max_bit_len().unwrap_or(1), RoundingMode::ToEven),
            cc: self.cc
        }
    }
}

impl<'a> Div<&'a Self> for AstroFloat {
    type Output = Self;

    fn div(self, rhs: &'a Self) -> Self::Output {
        AstroFloat {
            af: self.af.div(&rhs.af, self.af.mantissa_max_bit_len().unwrap_or(1), RoundingMode::ToEven),
            cc: self.cc
        }
    }
}

