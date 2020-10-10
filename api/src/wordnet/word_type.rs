use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum WordType {
    Noun,
    Verb,
    Adj,
    Adv,
}
