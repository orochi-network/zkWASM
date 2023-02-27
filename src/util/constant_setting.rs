pub trait ConstantValue {
    const ZERO: Self;
    const ONE: Self;
}

impl ConstantValue for u64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

pub const NUM_BYTES_FOR_LOCAL_GET: usize = 8;