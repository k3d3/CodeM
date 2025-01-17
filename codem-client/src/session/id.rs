use std::fmt;
use rand::seq::SliceRandom;

use self::words::{ADJECTIVES, NOUNS};

pub(crate) mod words;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(pub String);

impl SessionId {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        
        // Keep trying until we get a unique combination
        loop {
            let adj = ADJECTIVES.choose(&mut rng).unwrap();
            let noun = NOUNS.choose(&mut rng).unwrap();
            let id = format!("{}-{}", adj, noun);
            return Self(id);
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}