use std::collections::{BTreeMap, BTreeSet};

#[derive(Default, Debug)]
pub struct Model {
    pub text: String,
    pub indicators: IndicatorResults,
}

pub type IndicatorResults = BTreeMap<String, BTreeSet<Indicator>>;

#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Indicator {
    pub text: String,
    pub start_position: usize,
    pub end_position: usize,
}
