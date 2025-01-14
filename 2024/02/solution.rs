use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap};
use std::fmt::Write;
use std::iter;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Import<'a>(pub &'a [&'a str]);

pub enum Order {
    Original,
    Sorted,
}

pub fn format_flat(imports: &[Import], order: Order) -> Vec<String> {
    let cleaned = clean_up(imports, order);

    cleaned.iter().map(|import| import.0.join("::")).collect()
}

pub fn format_nested(imports: &[Import], order: Order) -> Vec<String> {
    let cleaned = clean_up(imports, order);

    let mut tree = Tree::new();
    for import in cleaned.iter() {
        tree.insert(import.0.iter().copied())
    }

    tree.visit()
}

fn clean_up<'a>(imports: &[Import<'a>], order: Order) -> Vec<Import<'a>> {
    match order {
        Order::Original => {
            let mut processed = BTreeSet::<_>::new();

            imports
                .iter()
                .filter(|&import| {
                    let newly_inserted = processed.insert(import);
                    newly_inserted
                })
                .map(|import| import.clone())
                .collect()
        }
        Order::Sorted => {
            let mut sorted: Vec<_> = imports.iter().map(|import| import.clone()).collect();

            sorted.sort();
            sorted.dedup();
            sorted
        }
    }
}

struct Tree<'a> {
    root: Node<'a>,
}

impl<'a> Tree<'a> {
    fn new() -> Self {
        Tree { root: Node::default() }
    }

    fn insert(&mut self, mut import: impl Iterator<Item = &'a str>) {
        let mut node = &mut self.root;

        while let Some(part) = import.next() {
            node = node.insert_node(part);
        }

        node.set_leaf();
    }

    fn visit(&self) -> Vec<String> {
        let mut result = vec![];

        for (key, node) in &self.root.children {
            let mut output = String::new();
            node.visit(key, 0, &mut output);

            output.pop();
            output.pop();
            output.push_str("\n");
            result.push(output)
        }

        result
    }
}

#[derive(Default)]
struct Node<'a> {
    has_value: bool,
    keys_set: HashMap<&'a str, usize>,
    children: Vec<(&'a str, Node<'a>)>,
}

impl<'a> Node<'a> {
    fn insert_node(&mut self, val: &'a str) -> &mut Node<'a> {
        match self.keys_set.entry(val) {
            Entry::Occupied(occupied) => {
                let index = occupied.get();
                &mut self.children[*index].1
            }
            Entry::Vacant(vacant) => {
                let next_index = self.children.len();
                self.children.push((val, Node::default()));
                vacant.insert(next_index);
                &mut self.children.last_mut().unwrap().1
            }
        }
    }

    fn set_leaf(&mut self) {
        self.has_value = true;
    }

    fn visit(&self, val: &str, depth: usize, output: &mut String) {
        let tab = "    ";
        let indent = iter::repeat(tab).take(depth).collect::<String>();

        if self.children.is_empty() {
            if self.has_value {
                write!(output, "{indent}{val},\n").unwrap();
            }
        } else {
            write!(output, "{indent}{val}::{{\n").unwrap();

            if self.has_value {
                write!(output, "{indent}{tab}self,\n").unwrap();
            }

            for (key, node) in &self.children {
                node.visit(key, depth + 1, output);
            }

            write!(output, "{indent}}},\n").unwrap();
        }
    }
}
