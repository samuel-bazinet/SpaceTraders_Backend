use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PVec<T: Display>(pub Vec<T>);

impl<T: Display> Display for PVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret_val = String::from("[\n");
        for i in self.0.iter() {
            ret_val += format!("{}\n", i).as_str();
        }
        ret_val += "]";
        write!(f, "{}", ret_val)
    }
}
