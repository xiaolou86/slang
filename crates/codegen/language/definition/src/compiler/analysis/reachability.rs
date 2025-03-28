use crate::{
    compiler::{analysis::Analysis, versions::VersionSet},
    spanned::TriviaParser,
    Identifier,
};
use std::collections::HashSet;

pub fn analyze_reachability(analysis: &mut Analysis) {
    check_unreachabable_items(analysis);
    check_unused_versions(analysis);
}

fn check_unused_versions(analysis: &mut Analysis) {
    for (name, metadata) in &analysis.metadata {
        if name == &*analysis.language.root_item {
            continue;
        }

        let unused_in = metadata.defined_in.difference(&metadata.used_in);

        if !unused_in.is_empty() {
            analysis
                .errors
                .add(&metadata.name, &Errors::UnusedVersion(name, &unused_in));
        }
    }
}

fn check_unreachabable_items(analysis: &mut Analysis) {
    let language = analysis.language.clone();

    let mut queue = vec![&*language.root_item];

    collect_trivia(&language.leading_trivia, &mut queue);
    collect_trivia(&language.trailing_trivia, &mut queue);

    let mut visited = queue.iter().cloned().collect::<HashSet<_>>();

    while let Some(name) = queue.pop() {
        for referenced_item in &analysis.metadata[name].referenced_items {
            if visited.insert(referenced_item) {
                queue.push(referenced_item);
            }
        }
    }

    for metadata in analysis.metadata.values() {
        if !metadata.defined_in.is_empty() && !visited.contains(&*metadata.name) {
            analysis
                .errors
                .add(&metadata.name, &Errors::Unreachable(&*metadata.name));
        }
    }
}

fn collect_trivia<'l>(parser: &'l TriviaParser, acc: &mut Vec<&'l Identifier>) {
    match parser {
        TriviaParser::Sequence { parsers } | TriviaParser::Choice { parsers } => {
            for parser in parsers {
                collect_trivia(parser, acc);
            }
        }
        TriviaParser::ZeroOrMore { parser } | TriviaParser::Optional { parser } => {
            collect_trivia(parser, acc);
        }
        TriviaParser::Trivia { trivia } => {
            acc.push(trivia);
        }
    };
}

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("Item '{0}' is not used in versions: {1}")]
    UnusedVersion(&'err Identifier, &'err VersionSet),
    #[error("Item '{0}' is not reachable from grammar root.")]
    Unreachable(&'err Identifier),
}
