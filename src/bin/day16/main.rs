use std::collections::HashSet;
use std::env::args;
use std::{collections::HashMap, ops::RangeInclusive};
use std::{fs, iter};

use anyhow::{anyhow, bail, Context};
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, newline, u64};
use nom::combinator::opt;
use nom::multi::{fold_many0, separated_list0};
use nom::sequence::{separated_pair, terminated, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let Some(path) = args().nth(1) else {
        bail!("please provide the path to input file")        
    };
    let (rules, my_ticket, nearby_tickets) = parse_input(&path)?;

    let mut error_rate = 0;
    let valid_tickets: Vec<_> = nearby_tickets
        .iter()
        .filter(|ticket| {
            let mut any_invalid = false;
            for n in ticket.iter() {
                if !rules.values().any(|r| r.contains(*n)) {
                    any_invalid = true;
                    error_rate += n;
                }
            }
            !any_invalid
        })
        .chain(iter::once(&my_ticket))
        .collect();

    println!("\nThe error rate is: {error_rate}");

    let n_rules = rules.len();
    let mut m: HashMap<String, HashSet<usize>> = HashMap::new();
    for (name, range) in rules {
        let entry = m.entry(name).or_insert(HashSet::from_iter(0..n_rules));
        for t in &valid_tickets {
            for (i, field) in t.iter().enumerate() {
                if !range.contains(*field) {
                    entry.remove(&i);
                }
            }
        }
    }

    let mut order = HashMap::new();
    let mut xs = Vec::new();
    while !m.is_empty() {
        m.retain(|n, s| {
            if s.len() == 1 {
                let x = s.iter().cloned().next().unwrap();
                xs.push(x);
                order.insert(n.clone(), x);
                return false;
            }
            true
        });
        for (_, s) in &mut m {
            for x in &xs {
                s.remove(&x);
            }
        }
        xs.clear();
    }

    let ans: u64 = order
        .iter()
        .filter_map(|(n, x)| {
            if n.starts_with("departure") {
                return Some(my_ticket[*x]);
            }
            None
        })
        .product();

    println!("The product of departure fields is: {ans}.");

    Ok(())
}

#[derive(Debug)]
struct ValidRanges {
    r1: RangeInclusive<u64>,
    r2: RangeInclusive<u64>,
}

type TicketRules = HashMap<String, ValidRanges>;

type Ticket = Vec<u64>;

impl ValidRanges {
    fn contains(&self, n: u64) -> bool {
        self.r1.contains(&n) || self.r2.contains(&n)
    }
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    let mut parser = separated_pair(u64, char('-'), u64);
    parser(input).map(|(rest, (start, end))| (rest, start..=end))
}

fn parse_rule(input: &str) -> IResult<&str, (&str, ValidRanges)> {
    terminated(
        tuple((terminated(is_not(":"), tag(": ")), parse_ranges)),
        opt(newline),
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, TicketRules> {
    fold_many0(parse_rule, HashMap::new, |mut map, (name, ranges)| {
        map.insert(name.to_owned(), ranges);
        map
    })(input)
}

fn parse_ranges(input: &str) -> IResult<&str, ValidRanges> {
    let mut parser = separated_pair(parse_range, tag(" or "), parse_range);
    parser(input).map(|(rest, (r1, r2))| (rest, ValidRanges { r1, r2 }))
}

fn parse_ticket(input: &str) -> IResult<&str, Ticket> {
    separated_list0(char(','), u64)(input)
}

fn parse_tickets(input: &str) -> IResult<&str, Vec<Ticket>> {
    separated_list0(char('\n'), parse_ticket)(input)
}

fn parse_input(path: &str) -> anyhow::Result<(TicketRules, Ticket, Vec<Ticket>)> {
    let input = fs::read_to_string(path)?;
    let sections = input.split("\n\n").map(String::from).collect::<Vec<_>>();
    assert_eq!(sections.len(), 3);

    let rules = parse_rules(&sections[0])
        .map_err(|e| anyhow!("parsing the rules failed: {}", e))?
        .1;

    let (_header, my_ticket) = &sections[1]
        .split_once('\n')
        .context("missing section header")?;

    let my_ticket = parse_ticket(my_ticket)
        .map_err(|_| anyhow!("error parsing ticket"))?
        .1;

    let (_header, tickets) = &sections[2]
        .split_once('\n')
        .context("missing section header")?;

    let tickets = parse_tickets(tickets)
        .map_err(|e| anyhow!("error parsing nearby ticket list: {}", e))?
        .1;

    Ok((rules, my_ticket, tickets))
}
