//! Wrapper for astro_float::BigFloatNumber

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
    cell::RefCell,
    rc::Rc,
};
use astro_float::{BigFloatNumber, Consts};
use astro_float::Exponent;
use astro_float::RoundingMode;
use astro_float::Radix;

pub struct AstroFloat
{
    af: BigFloatNumber,
    pub cc: Rc<RefCell<Consts>>,
}


impl AstroFloat {

    pub fn random_normal(exp_shift: Exponent, exp_range: Exponent, cc: Rc<RefCell<Consts>>) -> Self {
        AstroFloat {
            af: BigFloatNumber::random_normal(132, exp_shift, exp_shift + exp_range).unwrap(),
            cc,
        }
    }

    pub fn new(f: BigFloatNumber, cc: Rc<RefCell<Consts>>) -> Self {
        AstroFloat {
            af: f,
            cc,
        }
    }

    pub fn inner(&self) -> &BigFloatNumber {
        &self.af
    }
}


impl Display for AstroFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.af.format(Radix::Dec, RoundingMode::ToEven).unwrap())
    }
}

impl Clone for AstroFloat {
    fn clone(&self) -> Self {
        Self {
            af: self.af.clone().unwrap(),
            cc: self.cc.clone(),
        }
    }
}

impl Add<Self> for AstroFloat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        AstroFloat {
            af: self.af.add(&rhs.af, RoundingMode::ToEven).unwrap(),
            cc: self.cc
        }
    }
}

impl<'a> Add<&'a Self> for AstroFloat {
    type Output = Self;

    fn add(self, rhs: &'a Self) -> Self::Output {
        AstroFloat {
            af: self.af.add(&rhs.af, RoundingMode::ToEven).unwrap(),
            cc: self.cc
        }
    }
}

impl Sub<Self> for AstroFloat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        AstroFloat {
            af: self.af.sub(&rhs.af, RoundingMode::ToEven).unwrap(),
            cc: self.cc
        }
    }
}

impl<'a> Sub<&'a Self> for AstroFloat {
    type Output = Self;

    fn sub(self, rhs: &'a Self) -> Self::Output {
        AstroFloat {
            af: self.af.sub(&rhs.af, RoundingMode::ToEven).unwrap(),
            cc: self.cc
        }
    }
}

impl Mul<Self> for AstroFloat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        AstroFloat {
            af: self.af.mul(&rhs.af, RoundingMode::ToEven).unwrap(),
            cc: self.cc
        }
    }
}

impl<'a> Mul<&'a Self> for AstroFloat {
    type Output = Self;

    fn mul(self, rhs: &'a Self) -> Self::Output {
        AstroFloat {
            af: self.af.mul(&rhs.af, RoundingMode::ToEven).unwrap(),
            cc: self.cc
        }
    }
}

impl Div<Self> for AstroFloat {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        AstroFloat {
            af: self.af.div(&rhs.af, RoundingMode::ToEven).unwrap(),
            cc: self.cc
        }
    }
}

impl<'a> Div<&'a Self> for AstroFloat {
    type Output = Self;

    fn div(self, rhs: &'a Self) -> Self::Output {
        AstroFloat {
            af: self.af.div(&rhs.af, RoundingMode::ToEven).unwrap(),
            cc: self.cc
        }
    }
}

