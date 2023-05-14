use std::fmt::Display;

use serde::{Deserialize, Serialize};

mod agent_response;
pub mod response_fields {
    pub use super::agent_response::AgentResponseFields;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResponse<T> {
    data: T,
}

impl<T: Display> Display for JsonResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}
