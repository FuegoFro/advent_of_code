use num_traits::Signed;

pub trait ZeroOneConst {
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! impl_zero_one_const {
    ($t:ty, $zero:literal, $one:literal) => {
        impl ZeroOneConst for $t {
            const ZERO: Self = $zero;
            const ONE: Self = $one;
        }
    };
}

pub trait NegOneConst: Signed {
    const NEG_ONE: Self;
}

macro_rules! impl_neg_one_const {
    ($t:ty, $zero:literal, $one:literal, "signed") => {
        impl NegOneConst for $t {
            const NEG_ONE: Self = $zero - $one;
        }
    };
    ($t:ty, $zero:literal, $one:literal, "unsigned") => {};
}

pub trait CheckedOps: Sized {
    fn checked_add(&self, v: &Self) -> Option<Self>;
    fn checked_sub(&self, v: &Self) -> Option<Self>;
}

macro_rules! impl_checked_ops_integer {
    ($t:ty) => {
        impl CheckedOps for $t {
            fn checked_add(&self, v: &Self) -> Option<Self> {
                <$t as num_traits::CheckedAdd>::checked_add(self, v)
            }
            fn checked_sub(&self, v: &Self) -> Option<Self> {
                <$t as num_traits::CheckedSub>::checked_sub(self, v)
            }
        }
    };
}

macro_rules! impl_checked_ops_float {
    ($t:ty) => {
        impl CheckedOps for $t {
            fn checked_add(&self, v: &Self) -> Option<Self> {
                Some(self + v)
            }
            fn checked_sub(&self, v: &Self) -> Option<Self> {
                Some(self - v)
            }
        }
    };
}

macro_rules! impl_checked_ops {
    ($t:ty, "integer") => {
        impl_checked_ops_integer!($t);
    };
    ($t:ty, "float") => {
        impl_checked_ops_float!($t);
    };
}

// Impl for all types

macro_rules! impl_num_ops {
    ($t:ty, $zero:literal, $one:literal, $is_signed:tt, $is_integer:tt) => {
        impl_zero_one_const!($t, $zero, $one);
        impl_neg_one_const!($t, $zero, $one, $is_signed);
        impl_checked_ops!($t, $is_integer);
    };
}

impl_num_ops!(usize, 0usize, 1usize, "unsigned", "integer");
impl_num_ops!(u8, 0u8, 1u8, "unsigned", "integer");
impl_num_ops!(u16, 0u16, 1u16, "unsigned", "integer");
impl_num_ops!(u32, 0u32, 1u32, "unsigned", "integer");
impl_num_ops!(u64, 0u64, 1u64, "unsigned", "integer");
impl_num_ops!(u128, 0u128, 1u128, "unsigned", "integer");
impl_num_ops!(isize, 0isize, 1isize, "signed", "integer");
impl_num_ops!(i8, 0i8, 1i8, "signed", "integer");
impl_num_ops!(i16, 0i16, 1i16, "signed", "integer");
impl_num_ops!(i32, 0i32, 1i32, "signed", "integer");
impl_num_ops!(i64, 0i64, 1i64, "signed", "integer");
impl_num_ops!(i128, 0i128, 1i128, "signed", "integer");
impl_num_ops!(f32, 0f32, 1f32, "signed", "float");
impl_num_ops!(f64, 0f64, 1f64, "signed", "float");
