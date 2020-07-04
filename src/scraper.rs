use crate::model::{Indicator, IndicatorResults};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeSet;

const IPV4_RE_STR: &str = r"(^|\s)((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)($|\s)";
const FQDN_REGEX_STR: &str = r"([a-zA-Z0-9-]{0,62}[a-zA-Z0-9]\.)+[a-zA-Z]{2,63}";
const MD5_REGEX_STR: &str = r"\b[a-fA-F0-9]{32}\b";
const SHA256_REGEX_STR: &str = r"\b[a-fA-F0-9]{64}\b";

lazy_static! {
    static ref IPV4_RE: Regex = Regex::new(IPV4_RE_STR).unwrap();
    static ref FQDN_RE: Regex = Regex::new(FQDN_REGEX_STR).unwrap();
    static ref MD5_RE: Regex = Regex::new(MD5_REGEX_STR).unwrap();
    static ref SHA256_RE: Regex = Regex::new(SHA256_REGEX_STR).unwrap();
}

pub fn scrape_text(text: &str) -> IndicatorResults {
    let mut res = IndicatorResults::new();
    res.insert(
        String::from("IPv4 Address"),
        scrape_for_regex(text, &IPV4_RE),
    );
    res.insert(String::from("FQDN"), scrape_for_regex(text, &FQDN_RE));
    res.insert(String::from("MD5"), scrape_for_regex(text, &MD5_RE));
    res.insert(String::from("SHA256"), scrape_for_regex(text, &SHA256_RE));
    res
}

fn scrape_for_regex(text: &str, re: &Regex) -> BTreeSet<Indicator> {
    re.find_iter(text)
        .map(|m| Indicator {
            text: String::from(m.as_str()),
            start_position: m.start(),
            end_position: m.end(),
        })
        .collect()
}
