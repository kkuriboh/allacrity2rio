use std::collections::HashMap;

use serde::Deserialize;
use serde_yaml::Mapping;

#[derive(Deserialize)]
pub struct AlacrittyConf {
    pub colors: HashMap<String, ColorsOrIndexed>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ColorsOrIndexed {
    Colors(HashMap<String, String>),
    IndexedColors(Vec<Mapping>),
    Hints(HashMap<String, Mapping>),
}
