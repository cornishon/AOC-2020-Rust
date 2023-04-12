use anyhow::anyhow;
use std::{collections::HashMap, fs, num::ParseIntError, str::FromStr};
use thiserror::Error;

fn main() -> anyhow::Result<()> {
    let path = "src/bin/day19/test_input.txt";
    let input = fs::read_to_string(path)?;
    let (rule_str, messages) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("expected the input to consist of rules and messages section"))?;

    let mut rules: HashMap<u32, Rule> = HashMap::new();
    for rule in rule_str.lines() {
        let (id, rule) = rule
            .split_once(": ")
            .ok_or_else(|| anyhow!("the rule entry must have the format of `id: rule`"))?;
        rules.insert(id.parse()?, rule.parse()?);
    }
    dbg!(&rules);

    Ok(())
}

#[derive(Debug)]
enum Rule {
    Ref(u32),
    Lit(String),
    Seq(Vec<Rule>),
    Alt(Vec<Rule>, Vec<Rule>),
}

#[derive(Debug, Error)]
enum ParseRuleError {
    #[error("invalid index")]
    Index(#[from] ParseIntError),
}

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = s.parse::<u32>() {
            return Ok(Rule::Ref(i));
        }

        if s.starts_with('"') && s.ends_with('"') {
            return Ok(Rule::Lit(s[1..s.len() - 1].to_owned()));
        }

        let try_into_vec = |s: &str| -> Result<Vec<Rule>, Self::Err> {
            let mut v = Vec::<Rule>::new();
            for r in s.split_whitespace() {
                v.push(r.parse()?);
            }
            Ok(v)
        };

        if let Some((lhs, rhs)) = s.split_once(" | ") {
            let lhs = try_into_vec(lhs)?;
            let rhs = try_into_vec(rhs)?;
            Ok(Rule::Alt(lhs, rhs))
        } else {
            Ok(Rule::Seq(try_into_vec(s)?))
        }
    }
}
