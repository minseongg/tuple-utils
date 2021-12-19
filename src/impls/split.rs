#![allow(clippy::all)]
pub trait SplitFirst {
    type Left;
    type Right;
    #[doc = r" Returns splitted tuples."]
    fn split_first(self) -> (Self::Left, Self::Right);
}
impl<T0> SplitFirst for (T0,) {
    type Left = T0;
    type Right = ();
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, ())
    }
}
impl<T0, T1> SplitFirst for (T0, T1) {
    type Left = T0;
    type Right = (T1,);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1,))
    }
}
impl<T0, T1, T2> SplitFirst for (T0, T1, T2) {
    type Left = T0;
    type Right = (T1, T2);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2))
    }
}
impl<T0, T1, T2, T3> SplitFirst for (T0, T1, T2, T3) {
    type Left = T0;
    type Right = (T1, T2, T3);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3))
    }
}
impl<T0, T1, T2, T3, T4> SplitFirst for (T0, T1, T2, T3, T4) {
    type Left = T0;
    type Right = (T1, T2, T3, T4);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4))
    }
}
impl<T0, T1, T2, T3, T4, T5> SplitFirst for (T0, T1, T2, T3, T4, T5) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> SplitFirst for (T0, T1, T2, T3, T4, T5, T6) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> SplitFirst for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Left = T0;
    type Right = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn split_first(self) -> (Self::Left, Self::Right) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
pub trait SplitLast {
    type Left;
    type Right;
    #[doc = r" Returns splitted tuples."]
    fn split_last(self) -> (Self::Left, Self::Right);
}
impl<T0> SplitLast for (T0,) {
    type Left = ();
    type Right = T0;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((), self.0)
    }
}
impl<T0, T1> SplitLast for (T0, T1) {
    type Left = (T0,);
    type Right = T1;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0,), self.1)
    }
}
impl<T0, T1, T2> SplitLast for (T0, T1, T2) {
    type Left = (T0, T1);
    type Right = T2;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1), self.2)
    }
}
impl<T0, T1, T2, T3> SplitLast for (T0, T1, T2, T3) {
    type Left = (T0, T1, T2);
    type Right = T3;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2), self.3)
    }
}
impl<T0, T1, T2, T3, T4> SplitLast for (T0, T1, T2, T3, T4) {
    type Left = (T0, T1, T2, T3);
    type Right = T4;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3), self.4)
    }
}
impl<T0, T1, T2, T3, T4, T5> SplitLast for (T0, T1, T2, T3, T4, T5) {
    type Left = (T0, T1, T2, T3, T4);
    type Right = T5;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4), self.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> SplitLast for (T0, T1, T2, T3, T4, T5, T6) {
    type Left = (T0, T1, T2, T3, T4, T5);
    type Right = T6;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5), self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Left = (T0, T1, T2, T3, T4, T5, T6);
    type Right = T7;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7);
    type Right = T8;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    type Right = T9;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    type Right = T10;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    type Right = T11;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    type Right = T12;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    type Right = T13;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    type Right = T14;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    type Right = T15;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    type Right = T16;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    type Right = T17;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    type Right = T18;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    type Right = T19;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    type Right = T20;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    type Right = T21;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    type Right = T22;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    type Right = T23;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    type Right = T24;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    type Right = T25;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    type Right = T26;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    type Right = T27;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    type Right = T28;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    type Right = T29;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28), self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    type Right = T30;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> SplitLast for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Left = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    type Right = T31;
    fn split_last(self) -> (Self::Left, Self::Right) {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30), self.31)
    }
}
