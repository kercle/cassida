use std::fmt;

type BitMaskBase = u8;

pub struct BitMask(BitMaskBase);

impl BitMask {
    pub fn new(capacity: usize) -> Self {
        let max_cap = std::mem::size_of::<BitMaskBase>() * 8;
        assert!(capacity <= max_cap, "Allow for bit masks larger than {max_cap} bits.");
        Self(0)
    }

    pub fn set(&self, i: usize) -> Self {
        Self(self.0 | (1 << i))
    }

    pub fn is_set(&self, i: usize) -> bool {
        self.0 & (1 << i) != 0
    }

    pub fn is_clear(&self, i: usize) -> bool {
        !self.is_set(i)
    }

    pub fn count_unmatched(&self, total: usize) -> usize {
        total - self.0.count_ones() as usize
    }
}

impl fmt::Debug for BitMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bits = std::mem::size_of::<BitMaskBase>() * 8;
        write!(f, "BitMask({:0width$b})", self.0, width = bits)
    }
}
