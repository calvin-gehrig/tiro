use std::collections::{HashSet, VecDeque};

use crate::{
    grammar::{
        Rule,
        Symbol,
        RuleId,
        Grammar
    },
    lexer::Token
};

#[derive(Clone, Debug)]
pub struct EarleyItem {
    rule: Rule,
    rule_id: RuleId,
    start: usize,
    parse_position: usize
}

impl EarleyItem {
    fn new(rule: Rule, rule_id: RuleId, start: usize) -> Self {
        Self {
            rule,
            rule_id,
            start,
            parse_position: 0
        }
    }
    fn current_symbol(&self) -> Option<Symbol> {
        self.rule.get(self.parse_position).clone()
    }
    fn move_forward(mut self) -> Self {
        self.parse_position += 1;
        self
    }
    fn complete(self, chart: &mut Chart) -> Vec<EarleyItem> {
        if self.parse_position != self.rule.len() {
            panic!("Earley parser error");
        }
        chart.get_waiting_items(self.start, self.rule_id)
    }
}

struct StateSet {
    set: HashSet<RuleId>,
    queue: VecDeque<EarleyItem>,
    waiting_items: Vec<EarleyItem>
}

impl StateSet {
    fn new() -> Self {
        Self {
            set: HashSet::new(),
            queue: VecDeque::new(),
            waiting_items: Vec::new()
        }
    }
    fn starting_set(rule: &Rule) -> Self {
        let new_set = Self::new();
        new_set.queue.push_back(rule);
        new_set
    }

    fn add_new_rule(&mut self, rule_id: RuleId, current_position: usize, grammar: &Grammar) {
        if self.set.contains(rule_id) { return }

        self.set.insert(rule_id);
        for rule in grammar.get_rules(rule_id) {
            self.queue.push_back(
                EarleyItem::new(
                    rule,
                    rule_id,
                    current_position)
            );
        }
    }
    fn add_item(&mut self, item: EarleyItem) {
        self.queue.push_back(item)
    }
}

struct Chart {
    parse_result: Vec<EarleyItem>,
    waiting_set: Vec<Vec<EarleyItem>>
}

impl Chart {
    fn new() -> Self {
        Self {
            parse_result: Vec::new(),
            waiting_set: Vec::new()
        }
    }
    fn add_waiting_item(&mut self, item: EarleyItem) {
        if let Some(set) = self.waiting_set.last_mut() {
            set.push(item);
        } else { panic!("Earley parser error") }
    }
    fn get_waiting_items(&mut self, index: usize, rule_id: &RuleId) {
        if let Some(set) = self.waiting_set.get_mut(index) {
            set.iter().enumerate()
            .filter_map(|(index, item)| {
                if item.rule_id == rule_id {
                    Some(index) 
                } else { None }
            }).collect().iter().map(|index| {
                set.remove(index)
            }).collect()
        } else {
            panic!("Earley parser error")
        }
    }
    fn add_result(&mut self, item: EarleyItem) {
        self.parse_result.push(item);
    }
    fn return_result(self) -> Vec<EarleyItem> {
        self.parse_result
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<EarleyItem> {
    let grammar = Grammar::new();
    let mut chart = Vec::new();
    let mut current_set = StateSet::starting_set(grammar.starting_rule());
    let mut parse_result = Vec::new();

    for current_token in tokens {
        let next_set = StateSet::new();

        while let Some(item) = current_set.pop_front()  {
            match item.current_symbol() {

                Some(Symbol::NonTerminal(symbol)) => predict(symbol, item, &mut current_set, &mut chart, &grammar),
                   
                Some(Symbol::Terminal(token)) => scan(token, &current_token, item, &mut next_set),

                None => complete(item, &mut chart, &mut next_set, &mut parse_result)
            }
        }

        current_set = next_set;
    }
    chart.return_result()
}

fn predict(symbol: RuleId, item: EarleyItem current_set: &mut StateSet, chart: &mut Chart, grammar: &Grammar) {
    let current_position = chart.len() - 1;
    current_set.add_new_rule(symbol, current_position, &grammar);
    chart.add_waiting_item(item);
}

fn scan(token: Token, current_token: &Token, item: EarleyItem, next_set: &mut StateSet) {
    if match (token, current_token) {
        (Id(_), Id(_)) => true,
        (StringToken(_), StringToken(_)) => true,
        (Number(_), Number(_)) => true,
        (first, second) => first == second
    } {
        next_set.add_item(item.move_forward());
    }
}

fn complete(item: EarleyItem, chart: &mut Chart, next_set: &mut StateSet, parse_result: &mut Vec<EarleyItem>) {
    chart.add_result(item.clone());
    item.complete().into_iter()
        .for_each(|item| {
            next_set.add_item(item.move_forward());
        }).collect();
}
