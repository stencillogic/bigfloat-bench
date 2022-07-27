use crate::number::Number;


pub(crate) fn add_sub<T: Number>(n: &[T]) -> T {
    let mut f = n[0].clone();
    for i in 1..n.len() {
        if i & 1 == 0 {
            f = f.add(&n[i]);
        } else {
            f = f.sub(&n[i]);
        }
    }
    f
}

pub(crate) fn mul_div<T: Number>(n: &[T]) -> T {
    let mut f = n[0].clone();
    for i in 1..n.len()/2 {
        f = n[i*2].clone();
        if i & 1 == 0 {
            f = f.mul(&n[i*2+1]);
        } else {
            f = f.div(&n[i*2+1]);
        }
    }
    f
}

pub(crate) fn sqrt<T: Number>(n: &[T]) -> T {
    let mut f = n[0].sqrt();
    for i in 1..n.len() {
        f = n[i].sqrt();
    }
    f
}

pub(crate) fn cbrt<T: Number>(n: &[T]) -> T {
    let mut f = n[0].cbrt();
    for i in 1..n.len() {
        f = n[i].cbrt();
    }
    f
}

pub(crate) fn ln<T: Number>(n: &[T]) -> T {
    let mut f = n[0].ln();
    for i in 1..n.len() {
        f = n[i].ln();
    }
    f
}

pub(crate) fn exp<T: Number>(n: &[T]) -> T {
    let mut f = n[0].exp();
    for i in 1..n.len() {
        f = n[i].exp();
    }
    f
}

pub(crate) fn pow<T: Number>(n: &[T], n2: &[T]) -> T {
    let mut f = n[0].pow(&n2[0]);
    let mut j = 0;
    for i in 1..n.len() {
        j += 1;
        if j >= n2.len() {
            j = 0;
        }
        f = n[i].pow(&n2[j]);
    }
    f
}

pub(crate) fn sin_asin<T: Number>(n: &[T]) -> T {
    let mut f = n[0].sin();
    f = f.asin();
    for i in 1..n.len() {
        f = n[i].sin();
        f = f.asin();
    }
    f
}

pub(crate) fn cos_acos<T: Number>(n: &[T]) -> T {
    let mut f = n[0].cos();
    f = f.acos();
    for i in 1..n.len() {
        f = n[i].cos();
        f = f.acos();
    }
    f
}

pub(crate) fn tan_atan<T: Number>(n: &[T]) -> T {
    let mut f = n[0].tan();
    f = f.atan();
    for i in 1..n.len() {
        f = n[i].tan();
        f = f.atan();
    }
    f
}

pub(crate) fn sinh_asinh<T: Number>(n: &[T]) -> T {
    let mut f = n[0].sinh();
    f = f.asinh();
    for i in 1..n.len() {
        f = n[i].sinh();
        f = f.asinh();
    }
    f
}

pub(crate) fn cosh_acosh<T: Number>(n: &[T]) -> T {
    let mut f = n[0].cosh();
    f = f.acosh();
    for i in 1..n.len() {
        f = n[i].cosh();
        f = f.acosh();
    }
    f
}

pub(crate) fn tanh_atanh<T: Number>(n: &[T]) -> T {
    let mut f = n[0].tanh();
    f = f.atanh();
    for i in 1..n.len() {
        f = n[i].tanh();
        f = f.atanh();
    }
    f
}