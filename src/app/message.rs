#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Message {
    Get { index: usize },
    Set { index: usize, value: u8 },
}

impl Message {
    pub const fn get(index: usize) -> Self {
        Self::Get { index }
    }

    pub const fn set(index: usize, value: u8) -> Self {
        Self::Set { index, value }
    }
}
