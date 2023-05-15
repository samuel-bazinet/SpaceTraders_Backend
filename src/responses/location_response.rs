use std::{fmt::Display, collections::HashMap};

use serde::{Deserialize, Serialize};

//use crate::utils::printable_data_struct::{PHashMap, PVec};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct LocationResponseFields {
    systemSymbol: String,
    symbol: String,
    r#type: String,
    x: i64,
    y: i64,
    orbitals: Vec<HashMap<String, String>>,
    traits: Vec<HashMap<String, String>>,
    chart: HashMap<String, String>,
    faction: HashMap<String, String>,
}

impl Display for LocationResponseFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Location Fields:\n\tSystem Symbol: {}\n\tSymbol: {}\n\tType: {}\n\tX: {}\n\tY: {}\n\tOrbitals: {:?}\n\tTraits: {:?}\n\tChart: {:?}\n\tFaction: {:?}",
            self.systemSymbol, self.symbol, self.r#type, self.x, self.y, self.orbitals, self.traits, self.chart, self.faction
        )
    }
}

