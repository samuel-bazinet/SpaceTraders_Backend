use std::{collections::HashMap, fmt::Display, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PHashMap<K: Display + Eq + Hash, V: Display>(pub HashMap<K, V>);

impl<K: Display + Eq + Hash, V: Display> Display for PHashMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret_val = String::new();
        for i in self.0.iter() {
            ret_val += format!("{{{}, {}}}\n", i.0, i.1).as_str();
        }
        write!(f, "{}", ret_val)
    }
}
