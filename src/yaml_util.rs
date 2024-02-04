use glob::glob;
use std::collections::HashMap;

use crate::rules::Rule;

pub trait YamlDeserialize {
    fn from_yaml_file(path: &std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: std::marker::Sized;
}

pub fn load_staleness_rules() -> crate::types::StalenessRules {
    tracing::debug!("Loading rules.yaml");
    let mut resvec: Vec<Rule> = Default::default();
    for entry in glob("src/yaml/**/rules.yaml").unwrap() {
        resvec.extend(Vec::<Rule>::from_yaml_file(&entry.unwrap()).unwrap())
    }
    tracing::debug!("Filtering for StalenessRules");
    let mut staleness_rules: HashMap<String, u32> = Default::default();
    for rule in resvec {
        match rule {
            Rule::StalenessRule {
                name,
                seconds_till_stale,
            } => {
                staleness_rules.insert(name, seconds_till_stale);
            }
            _ => (),
        }
    }
    staleness_rules
}
