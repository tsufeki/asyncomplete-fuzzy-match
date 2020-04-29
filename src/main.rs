use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::iter::repeat;
use std::vec::Vec;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_writer, Value};

#[derive(Deserialize)]
struct Completions {
    id: i64,
    pattern: String,
    lists: Vec<CompletionList>,
}

#[derive(Deserialize)]
struct CompletionList {
    items: Vec<CompletionItem>,
    priority: i64,
}

#[derive(Serialize, Deserialize)]
struct CompletionItem {
    word: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    abbr: Option<String>,

    #[serde(flatten)]
    rest: HashMap<String, Value>,
}

struct Ranked<'a, T> {
    item: &'a T,
    priority: i64,
    rank: i64,
}

struct Ranker<'a, M: FuzzyMatcher> {
    pattern: &'a str,
    matcher: M,
}

impl<'a, M: FuzzyMatcher> Ranker<'a, M> {
    pub fn new(pattern: &'a str, matcher: M) -> Self {
        Ranker { pattern, matcher }
    }

    pub fn rank_item<'b>(
        &self,
        item: &'b CompletionItem,
        priority: i64,
    ) -> Option<Ranked<'b, CompletionItem>> {
        let text = item.abbr.as_deref().unwrap_or(&item.word);
        self.matcher
            .fuzzy_match(text, self.pattern)
            .map(|rank| Ranked {
                item,
                rank,
                priority,
            })
    }
}

#[derive(Serialize)]
struct FilteredCompletions<'a> {
    id: i64,
    items: Vec<&'a CompletionItem>,
}

impl Completions {
    pub fn filter(&self) -> FilteredCompletions {
        let ranker = Ranker::new(&self.pattern, SkimMatcherV2::default());

        let mut ranked: Vec<Ranked<CompletionItem>> = self
            .lists
            .iter()
            .map(|list| (list.items.iter(), repeat(list.priority)))
            .flat_map(|(items, priorities)| items.zip(priorities))
            .filter_map(|(item, priority)| ranker.rank_item(item, priority))
            .collect();

        ranked.sort_unstable_by_key(|item| (-item.priority, -item.rank));

        FilteredCompletions {
            id: self.id,
            items: ranked.iter().map(|item| item.item).collect(),
        }
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
        output.flush().unwrap();
    }
}
