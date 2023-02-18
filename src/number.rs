use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub, Deref, DerefMut}, cell::RefCell,
    rc::Rc,
};
use astro_float::Consts;
use dashu_float::{round::mode::HalfEven, FBig};
use dashu_int::{IBig, UBig};
use rug::{rand::RandState, Float, ops::CompleteRound};
use rand::random;
use crate::astro::AstroFloat;

pub(crate) trait GlobalState {}

pub struct StubGlobalState {}

impl GlobalState for StubGlobalState {}

pub struct AstroGlobalState {
    cc: Rc<RefCell<Consts>>
}

impl GlobalState for AstroGlobalState {}

pub(crate) trait Number<G: GlobalState>
where
    Self: Sized,
    Self: Display,
    Self: Clone,
{
    fn rand_normal(n: usize, exp_from: i32, exp_to: i32, gs: G, sign_positive: bool) -> Vec<Self>;

    fn global_state() -> G;

    fn add(&self, rhs: &Self) -> Self;

    fn sub(&self, rhs: &Self) -> Self;

    fn mul(&self, rhs: &Self) -> Self;

    fn div(&self, rhs: &Self) -> Self;

    fn sqrt(&self) -> Self;

    fn cbrt(&self) -> Self;

    fn ln(&self) -> Self;

    fn exp(&self) -> Self;

    fn pow(&self, n: &Self) -> Self;
    
    fn sin(&self) -> Self;
        
    fn asin(&self) -> Self;

    fn cos(&self) -> Self;
        
    fn acos(&self) -> Self;

    fn tan(&self) -> Self;
        
    fn atan(&self) -> Self;

    fn sinh(&self) -> Self;
        
    fn asinh(&self) -> Self;

    fn cosh(&self) -> Self;
        
    fn acosh(&self) -> Self;

    fn tanh(&self) -> Self;
        
    fn atanh(&self) -> Self;
}

impl Number<StubGlobalState> for rug::Float {
    fn rand_normal(n: usize, exp_from: i32, exp_to: i32, _gs: StubGlobalState, sign_positive: bool) -> Vec<Self> {
        let mut rand = RandState::new();
        let mut ret = vec![];
        for _ in 0..n {
            let sign = if sign_positive || random::<i8>() & 1 == 0 {1} else {-1};
            let exp_range = exp_to - exp_from;
            let exp = (if exp_range != 0 {random::<i32>().abs() % exp_range} else {0}) + exp_from;
            let f = Float::with_val(132,Float::random_bits(&mut rand));
            let p = Float::with_val(132, Float::i_pow_u(10, exp.unsigned_abs()));
            ret.push(Float::with_val(132, if exp >= 0 { f*p*sign } else { f/p*sign }));
        }
        ret
    }

    fn global_state() -> StubGlobalState {
        StubGlobalState {}
    }

    fn add(&self, rhs: &Self) -> Self {
        <&Self as Add<&Self>>::add(self, rhs).complete(self.prec())
    }

    fn sub(&self, rhs: &Self) -> Self {
        <&Self as Sub<&Self>>::sub(self, rhs).complete(self.prec())
    }

    fn mul(&self, rhs: &Self) -> Self {
        <&Self as Mul<&Self>>::mul(self, rhs).complete(self.prec())
    }

    fn div(&self, rhs: &Self) -> Self {
        <&Self as Div<&Self>>::div(self, rhs).complete(self.prec())
    }

    fn sqrt(&self) -> Self {
        self.clone().sqrt()
    }

    fn cbrt(&self) -> Self {
        self.clone().cbrt()
    }

    fn ln(&self) -> Self {
        self.clone().ln()
    }

    fn exp(&self) -> Self {
        self.clone().exp()
    }

    fn pow(&self, n: &Self) -> Self {
        rug::ops::Pow::pow(self.clone(), n)
    }
    
    fn sin(&self) -> Self {
        self.clone().sin()
    }
        
    fn asin(&self) -> Self {
        self.clone().asin()
    }

    fn cos(&self) -> Self {
        self.clone().cos()
    }
        
    fn acos(&self) -> Self {
        self.clone().acos()
    }

    fn tan(&self) -> Self {
        self.clone().tan()
    }
        
    fn atan(&self) -> Self {
        self.clone().atan()
    }

    fn sinh(&self) -> Self {
        self.clone().sinh()
    }
        
    fn asinh(&self) -> Self{
        self.clone().asinh()
    }

    fn cosh(&self) -> Self{
        self.clone().cosh()
    }
        
    fn acosh(&self) -> Self{
        self.clone().acosh()
    }

    fn tanh(&self) -> Self{
        self.clone().tanh()
    }
        
    fn atanh(&self) -> Self {
        self.clone().atanh()
    }
}

impl Number<StubGlobalState> for num_bigfloat::BigFloat {
    fn rand_normal(n: usize, exp_from: i32, exp_to: i32, _gs: StubGlobalState, sign_positive: bool) -> Vec<Self> {
        let mut ret = vec![];
        for _ in 0..n {
            let mut mantissa = [0i16; 10];
            mantissa.iter_mut().for_each(|v| *v = (random::<u16>() % 10000) as i16);

            if mantissa[9] == 0 {
                mantissa[9] = 9999;
            }
            while mantissa[9] / 1000 == 0 {
                mantissa[9] *= 10;
            }
            let sign = if sign_positive || random::<i8>() & 1 == 0 {1} else {-1};
            let exp_range = exp_to - exp_from;
            let exp = (if exp_range != 0 {random::<i32>().abs() % exp_range} else {0}) + exp_from - 40;
            ret.push(num_bigfloat::BigFloat::from_raw_parts(mantissa, 40, sign, exp as i8));
        }
        ret
    }

    fn global_state() -> StubGlobalState {
        StubGlobalState {}
    }

    fn add(&self, rhs: &Self) -> Self {
        num_bigfloat::BigFloat::add(self, rhs)
    }

    fn sub(&self, rhs: &Self) -> Self {
        num_bigfloat::BigFloat::sub(self, rhs)
    }

    fn mul(&self, rhs: &Self) -> Self {
        num_bigfloat::BigFloat::mul(self, rhs)
    }

    fn div(&self, rhs: &Self) -> Self {
        num_bigfloat::BigFloat::div(self, rhs)
    }

    fn sqrt(&self) -> Self {
        self.sqrt()
    }

    fn cbrt(&self) -> Self {
        self.cbrt()
    }

    fn ln(&self) -> Self {
        self.ln()
    }

    fn exp(&self) -> Self {
        self.exp()
    }

    fn pow(&self, n: &Self) -> Self {
        num_bigfloat::BigFloat::pow(self, n)
    }
    
    fn sin(&self) -> Self {
        self.sin()
    }
        
    fn asin(&self) -> Self {
        self.asin()
    }

    fn cos(&self) -> Self {
        self.cos()
    }
        
    fn acos(&self) -> Self {
        self.acos()
    }

    fn tan(&self) -> Self {
        self.tan()
    }
        
    fn atan(&self) -> Self {
        self.atan()
    }

    fn sinh(&self) -> Self {
        self.sinh()
    }

    fn asinh(&self) -> Self {
        self.asinh()
    }

    fn cosh(&self) -> Self {
        self.cosh()
    }

    fn acosh(&self) -> Self {
        self.acosh()
    }

    fn tanh(&self) -> Self {
        self.tanh()
    }

    fn atanh(&self) -> Self {
        self.atanh()
    }
}



impl Number<AstroGlobalState> for AstroFloat {

    fn rand_normal(n: usize, exp_from: i32, exp_to: i32, gs: AstroGlobalState, sign_positive: bool) -> Vec<Self> {
        let mut ret = vec![];
        for _ in 0..n {
            ret.push(AstroFloat::random_normal(exp_from, exp_to, gs.cc.clone(), sign_positive));
        }
        ret
    }

    fn global_state() -> AstroGlobalState {
        AstroGlobalState {
            cc: Rc::new(RefCell::new(Consts::new().unwrap()))
        }
    }

    fn add(&self, rhs: &Self) -> Self {
        AstroFloat::new(self.inner().add(rhs.inner(), self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven), self.cc.clone())
    }

    fn sub(&self, rhs: &Self) -> Self {
        AstroFloat::new(self.inner().sub(rhs.inner(), self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven), self.cc.clone())
    }

    fn mul(&self, rhs: &Self) -> Self {
        AstroFloat::new(self.inner().mul(rhs.inner(), self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven), self.cc.clone())
    }

    fn div(&self, rhs: &Self) -> Self {
        AstroFloat::new(self.inner().div(rhs.inner(), self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven), self.cc.clone())
    }

    fn sqrt(&self) -> Self {
        AstroFloat::new(self.inner().sqrt(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven), self.cc.clone())
    }

    fn cbrt(&self) -> Self {
        AstroFloat::new(self.inner().cbrt(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven), self.cc.clone())
    }

    fn ln(&self) -> Self {
        AstroFloat::new(self.inner().ln(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }

    fn exp(&self) -> Self {
        AstroFloat::new(self.inner().exp(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }

    fn pow(&self, n: &Self) -> Self {
        AstroFloat::new(self.inner().pow(n.inner(), self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }
    
    fn sin(&self) -> Self {
        AstroFloat::new(self.inner().sin(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }
        
    fn asin(&self) -> Self {
        AstroFloat::new(self.inner().asin(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }

    fn cos(&self) -> Self {
        AstroFloat::new(self.inner().cos(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }
        
    fn acos(&self) -> Self {
        AstroFloat::new(self.inner().acos(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }

    fn tan(&self) -> Self {
        AstroFloat::new(self.inner().tan(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }
        
    fn atan(&self) -> Self {
        AstroFloat::new(self.inner().atan(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }

    fn sinh(&self) -> Self {
        AstroFloat::new(self.inner().sinh(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }

    fn asinh(&self) -> Self {
        AstroFloat::new(self.inner().asinh(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }

    fn cosh(&self) -> Self {
        AstroFloat::new(self.inner().cosh(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }

    fn acosh(&self) -> Self {
        AstroFloat::new(self.inner().acosh(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }

    fn tanh(&self) -> Self {
        AstroFloat::new(self.inner().tanh(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }

    fn atanh(&self) -> Self {
        AstroFloat::new(self.inner().atanh(self.inner().mantissa_max_bit_len().unwrap_or(1), astro_float::RoundingMode::ToEven, self.cc.deref().borrow_mut().deref_mut()), self.cc.clone())
    }
}


impl Number<StubGlobalState> for FBig<HalfEven, 2> {
    fn rand_normal(n: usize, exp_from: i32, exp_to: i32, _gs: StubGlobalState, sign_positive: bool) -> Vec<Self> {
        let mut ret = vec![];
        for _ in 0..n {

            let mut mantissa = [0u64; 3];
            mantissa.iter_mut().for_each(|v| *v = random());

            if mantissa[mantissa.len() - 1] == 0 {
                mantissa[mantissa.len() - 1] = u64::MAX;
            }
            while mantissa[mantissa.len() - 1] <= (u64::MAX >> 1) {
                mantissa[mantissa.len() - 1] <<= 1;
            }

            let exp_range = exp_to - exp_from;
            let sign = if sign_positive || random::<i8>() & 1 == 0 {dashu_int::Sign::Positive} else {dashu_int::Sign::Negative};
            let exp = (if exp_range != 0 {random::<i32>().abs() % exp_range} else {0}) - mantissa.len() as i32 * 64 + exp_from;
            let exp = exp as isize * 3321928095 / 1000000000;

            let m = UBig::from_words(&mantissa);
            let i = IBig::from_parts(sign, m);

            ret.push(FBig::from_parts(i, exp));
        }
        ret
    }

    fn global_state() -> StubGlobalState {
        StubGlobalState {  }
    }

    fn add(&self, rhs: &Self) -> Self {
        <&FBig<dashu_float::round::mode::HalfEven> as Add>::add(self, rhs)
    }

    fn sub(&self, rhs: &Self) -> Self {
        <&FBig<dashu_float::round::mode::HalfEven> as Sub>::sub(self, rhs)
    }

    fn mul(&self, rhs: &Self) -> Self {
        <&FBig<dashu_float::round::mode::HalfEven> as Mul>::mul(self, rhs)
    }

    fn div(&self, rhs: &Self) -> Self {
        <&FBig<dashu_float::round::mode::HalfEven> as Div>::div(self, rhs)
    }

    fn sqrt(&self) -> Self {
        let s = self.clone();
        FBig::<HalfEven, 2>::sqrt(&s)
    }

    fn cbrt(&self) -> Self {
        self.clone()
    }

    fn ln(&self) -> Self {
        FBig::<HalfEven, 2>::ln(self)
    }

    fn exp(&self) -> Self {
        FBig::<HalfEven, 2>::exp(self)
    }

    fn pow(&self, n: &Self) -> Self {
        let s = self.clone();
        FBig::<HalfEven, 2>::powf(&s, n)
    }

    fn sin(&self) -> Self {
        self.clone()
    }

    fn asin(&self) -> Self {
        self.clone()
    }

    fn cos(&self) -> Self {
        self.clone()
    }

    fn acos(&self) -> Self {
        self.clone()
    }

    fn tan(&self) -> Self {
        self.clone()
    }

    fn atan(&self) -> Self {
        self.clone()
    }

    fn sinh(&self) -> Self {
        self.clone()
    }

    fn asinh(&self) -> Self {
        self.clone()
    }

    fn cosh(&self) -> Self {
        self.clone()
    }

    fn acosh(&self) -> Self {
        self.clone()
    }

    fn tanh(&self) -> Self {
        self.clone()
    }

    fn atanh(&self) -> Self {
        self.clone()
    }
}