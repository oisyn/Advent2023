mod parser;
pub use parser::*;

mod input;
pub use input::*;

mod fieldview;
pub use fieldview::*;

pub fn to_str(b: &[u8]) -> &str {
    unsafe { std::str::from_utf8_unchecked(b) }
}

pub fn is_nl(c: u8) -> bool {
    c == b'\r' || c == b'\n'
}

pub fn gcd<T>(mut n: T, mut m: T) -> T
where
    T: Copy + Default + Eq + Ord + std::ops::RemAssign<T>,
{
    let zero = Default::default();
    while m != zero {
        if m < n {
            (n, m) = (m, n);
        }
        m %= n;
    }
    n
}

pub trait IncrementalIdentity {
    type Identity;
    fn increment() -> Self::Identity;
}

pub trait Increment: IncrementalIdentity + Copy + std::ops::AddAssign<Self::Identity> {
    fn pre_inc(&mut self) -> Self {
        let r = *self;
        *self += Self::increment();
        r
    }

    fn post_inc(&mut self) -> Self {
        *self += Self::increment();
        *self
    }
}

impl<T> Increment for T where T: IncrementalIdentity + Copy + std::ops::AddAssign<T::Identity> {}

pub trait Decrement: IncrementalIdentity + Copy + std::ops::SubAssign<Self::Identity> {
    fn pre_dec(&mut self) -> Self {
        let r = *self;
        *self -= Self::increment();
        r
    }

    fn post_dec(&mut self) -> Self {
        *self -= Self::increment();
        *self
    }
}

impl<T> Decrement for T where T: IncrementalIdentity + Copy + std::ops::SubAssign<T::Identity> {}

macro_rules! impl_additive_identities {
    ($($t:ty),+) => {
        $(
            impl IncrementalIdentity for $t {
                type Identity = $t;
                fn increment() -> Self::Identity {
                    1 as $t
                }
            }
        )+
    };
}

impl_additive_identities!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
