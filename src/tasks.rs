use crate::number::{Number, GlobalState};


pub(crate) fn task_for_two_args<G: GlobalState, T: Number<G>>(n: &[T], op: impl Fn(&T, &T) -> T) -> T {
    let mut f = n[0].clone();
    let (s1, s2) = n.split_at(n.len() / 2);
    for _ in 0..2 {
        for (u, v) in s1.iter().zip(s2) {
            f = op(u, v);
        }
    }
    f
}

pub(crate) fn task_for_one_arg<G: GlobalState, T: Number<G>>(n: &[T], op: impl Fn(&T) -> T) -> T {
    let mut f = n[0].clone();
    for v in n.iter() {
        f = op(v);
    }
    f
}
