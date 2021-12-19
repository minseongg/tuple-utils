#![allow(clippy::all)]
pub trait Arity {
    #[doc = r" Returns arity of a tuple."]
    fn arity() -> usize;
}
impl Arity for () {
    fn arity() -> usize {
        0
    }
}
impl<T0> Arity for (T0,) {
    fn arity() -> usize {
        1usize
    }
}
impl<T0, T1> Arity for (T0, T1) {
    fn arity() -> usize {
        2usize
    }
}
impl<T0, T1, T2> Arity for (T0, T1, T2) {
    fn arity() -> usize {
        3usize
    }
}
impl<T0, T1, T2, T3> Arity for (T0, T1, T2, T3) {
    fn arity() -> usize {
        4usize
    }
}
impl<T0, T1, T2, T3, T4> Arity for (T0, T1, T2, T3, T4) {
    fn arity() -> usize {
        5usize
    }
}
impl<T0, T1, T2, T3, T4, T5> Arity for (T0, T1, T2, T3, T4, T5) {
    fn arity() -> usize {
        6usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> Arity for (T0, T1, T2, T3, T4, T5, T6) {
    fn arity() -> usize {
        7usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Arity for (T0, T1, T2, T3, T4, T5, T6, T7) {
    fn arity() -> usize {
        8usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    fn arity() -> usize {
        9usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn arity() -> usize {
        10usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn arity() -> usize {
        11usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn arity() -> usize {
        12usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn arity() -> usize {
        13usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn arity() -> usize {
        14usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn arity() -> usize {
        15usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn arity() -> usize {
        16usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    fn arity() -> usize {
        17usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    fn arity() -> usize {
        18usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    fn arity() -> usize {
        19usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    fn arity() -> usize {
        20usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    fn arity() -> usize {
        21usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    fn arity() -> usize {
        22usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    fn arity() -> usize {
        23usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    fn arity() -> usize {
        24usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    fn arity() -> usize {
        25usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    fn arity() -> usize {
        26usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    fn arity() -> usize {
        27usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    fn arity() -> usize {
        28usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    fn arity() -> usize {
        29usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    fn arity() -> usize {
        30usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    fn arity() -> usize {
        31usize
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> Arity for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    fn arity() -> usize {
        32usize
    }
}
