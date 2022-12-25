use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum KeyType {
    Unlimited,
    Standard
}

// impl fmt::Display for KeyType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//         // or, alternatively:
//         // fmt::Debug::fmt(self, f)
//     }
// }

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyType::Unlimited => write!(f, "Unlimited"),
            KeyType::Standard => write!(f, "Standard"),
        }
    }
}