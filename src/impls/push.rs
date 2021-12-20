#![allow(clippy::all)]
pub trait PushFront {
    type Out<T>;
    #[doc = r" Returns the tuple after pushing."]
    fn push_front<T>(self, elt: T) -> Self::Out<T>;
}
impl PushFront for () {
    type Out<T> = (T,);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt,)
    }
}
impl<T0> PushFront for (T0,) {
    type Out<T> = (T, T0);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0)
    }
}
impl<T0, T1> PushFront for (T0, T1) {
    type Out<T> = (T, T0, T1);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1)
    }
}
impl<T0, T1, T2> PushFront for (T0, T1, T2) {
    type Out<T> = (T, T0, T1, T2);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2)
    }
}
impl<T0, T1, T2, T3> PushFront for (T0, T1, T2, T3) {
    type Out<T> = (T, T0, T1, T2, T3);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3)
    }
}
impl<T0, T1, T2, T3, T4> PushFront for (T0, T1, T2, T3, T4) {
    type Out<T> = (T, T0, T1, T2, T3, T4);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4)
    }
}
impl<T0, T1, T2, T3, T4, T5> PushFront for (T0, T1, T2, T3, T4, T5) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> PushFront for (T0, T1, T2, T3, T4, T5, T6) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> PushFront for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Out<T> = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn push_front<T>(self, elt: T) -> Self::Out<T> {
        (elt, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
pub trait PushBack {
    type Out<T>;
    #[doc = r" Returns the tuple after pushing."]
    fn push_back<T>(self, elt: T) -> Self::Out<T>;
}
impl PushBack for () {
    type Out<T> = (T,);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (elt,)
    }
}
impl<T0> PushBack for (T0,) {
    type Out<T> = (T0, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, elt)
    }
}
impl<T0, T1> PushBack for (T0, T1) {
    type Out<T> = (T0, T1, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, elt)
    }
}
impl<T0, T1, T2> PushBack for (T0, T1, T2) {
    type Out<T> = (T0, T1, T2, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, elt)
    }
}
impl<T0, T1, T2, T3> PushBack for (T0, T1, T2, T3) {
    type Out<T> = (T0, T1, T2, T3, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, elt)
    }
}
impl<T0, T1, T2, T3, T4> PushBack for (T0, T1, T2, T3, T4) {
    type Out<T> = (T0, T1, T2, T3, T4, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5> PushBack for (T0, T1, T2, T3, T4, T5) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> PushBack for (T0, T1, T2, T3, T4, T5, T6) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, elt)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> PushBack for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Out<T> = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T);
    fn push_back<T>(self, elt: T) -> Self::Out<T> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31, elt)
    }
}
