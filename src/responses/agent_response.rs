use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AgentResponseFields {
    accountId: String,
    symbol: String,
    headquarters: String,
    credits: i64,
}

impl Display for AgentResponseFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Agent Fields:\n\tAccountID: {}\n\tSymbol: {}\n\tHeadquarters: {}\n\tCredits: {}",
            self.accountId, self.symbol, self.headquarters, self.credits
        )
    }
}
