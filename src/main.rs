use fuzzy_matcher::skim::fuzzy_match;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_writer, Value};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::iter::repeat;
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
struct CompletionItem {
    word: String,

    #[serde(flatten)]
    rest: HashMap<String, Value>,
}

struct Ranked<'a, T> {
    item: &'a T,
    priority: i64,
    rank: i64,
}

fn rank_item<'a>(
    item: &'a CompletionItem,
    pattern: &str,
    priority: i64,
) -> Option<Ranked<'a, CompletionItem>> {
    let mut candidate: String = item.word.clone();
    let prefix: &str = &pattern[0..2];
    if let Some(Value::String(abbr)) = item.rest.get("abbr") {
        candidate = abbr.clone();
    }
    if !candidate.starts_with(prefix) {
        return None;
    }

    fuzzy_match(&candidate, &pattern).map(|rank| Ranked {
        item,
        rank,
        priority,
    })
}

#[derive(Deserialize, Debug)]
struct CompletionList {
    items: Vec<CompletionItem>,
    priority: i64,
}

#[derive(Deserialize, Debug)]
struct Completions {
    pattern: String,
    lists: Vec<CompletionList>,
}

impl Completions {
    pub fn filter(&self) -> Vec<&CompletionItem> {
        let mut ranked: Vec<Ranked<CompletionItem>> = self
            .lists
            .iter()
            .map(|list| (list.items.iter(), repeat(list.priority)))
            .flat_map(|(items, priorities)| items.zip(priorities))
            .filter_map(|(item, priority)| rank_item(&item, &self.pattern, priority))
            .collect();

        ranked.sort_unstable_by_key(|item| (-item.priority, -item.rank));

        ranked.iter().map(|item| item.item).collect()
    }
}

fn main() {
    let input = io::stdin();
    let mut output = io::stdout();

    for line in input.lock().lines() {
        let list = from_str::<Completions>(&line.unwrap());
        if let Ok(list) = list {
            let items = list.filter();
            to_writer(output.lock(), &items).unwrap();
        }
        output.write_all(b"\n").unwrap();
    }
}
