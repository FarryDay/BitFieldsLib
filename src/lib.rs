#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitField(u32);

impl BitField {
    pub fn new() -> Self {
        BitField(0)
    }

    pub fn all() -> Self {
        BitField(u32::MAX) // 0xFFFF_FFFF
    }

    pub fn set(&mut self, flag: u32) {
        self.0 |= flag;
    }

    pub fn unset(&mut self, flag: u32) {
        self.0 &= !flag;
    }

    pub fn contains(&self, flag: u32) -> bool {
        (self.0 & flag) == flag
    }

    pub fn toggle(&mut self, flag: u32) {
        self.0 ^= flag;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const READ: u32 = 1 << 0; // 0b001
    const WRITE: u32 = 1 << 1; // 0b010
    const ADMIN: u32 = 1 << 2; // 0b100 and so on...

    #[test]
    fn test_all() {
        let mut flags = BitField::all();
        assert_eq!(flags.0, u32::MAX);
    }

    #[test]
    fn test_set_and_unset() {
        let mut flags = BitField::new();

        //Test set flags
        flags.set(READ);
        assert_eq!(flags.0, 1);
        flags.set(WRITE);
        assert_eq!(flags.0, 3);

        //Test unset flags
        flags.unset(ADMIN);
        assert_eq!(flags.0, 3);
        flags.unset(READ);
        assert_eq!(flags.0, 2);
    }

    #[test]
    fn test_set_contains() {
        let mut flags = BitField::new();
        flags.set(READ);
        assert!(flags.contains(READ));
        assert!(!flags.contains(WRITE));
    }

    #[test]
    fn test_toggle() {
        let mut flags = BitField::new();
        flags.toggle(ADMIN);
        assert!(flags.contains(ADMIN));
        flags.toggle(ADMIN);
        assert!(!flags.contains(ADMIN));
    }
}
