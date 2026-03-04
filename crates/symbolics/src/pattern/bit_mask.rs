use std::fmt;

type BitMaskBase = u8;

#[derive(Clone)]
pub struct BitMask {
    mask: BitMaskBase,
    capacity: usize,
}

impl BitMask {
    pub fn new(capacity: usize) -> Self {
        let max_cap = std::mem::size_of::<BitMaskBase>() * 8;

        assert!(capacity >= 1, "Capacity needs to be at least 1.");

        assert!(
            capacity <= max_cap,
            "Allow for bit masks larger than {max_cap} bits."
        );

        Self { mask: 0, capacity }
    }

    pub fn set(&mut self, i: usize) {
        self.mask = (self.mask | (1 << i)) & ((1 << self.capacity) - 1);
    }

    pub fn is_set(&self, i: usize) -> bool {
        self.mask & (1 << i) != 0
    }

    pub fn is_full(&self) -> bool {
        self.count_matched() == self.capacity
    }

    pub fn count_matched(&self) -> usize {
        self.mask.count_ones() as usize
    }

    pub fn count_unmatched(&self) -> usize {
        self.capacity - self.count_matched()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl fmt::Debug for BitMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bits = std::mem::size_of::<BitMaskBase>() * 8;
        let bits_string = format!("{:0width$b}", self.mask, width = bits);
        write!(f, "BitMask({})", &bits_string[bits_string.len() - self.capacity..])
    }
}
