use indexmap::{IndexMap, IndexSet};

use crate::{
    types::LanguageDefinitionRef,
    validation::visitors::{LocationRef, Reporter, VersionSet},
};

pub struct Entry {
    location: LocationRef,
    references: IndexSet<String>,
    defined_in: VersionSet,
    used_in: VersionSet,
}

pub struct Metadata {
    productions: IndexMap<String, Entry>,
}

impl Metadata {
    pub fn new() -> Self {
        return Self {
            productions: IndexMap::new(),
        };
    }

    pub fn add_production(&mut self, production: &str, location: &LocationRef) {
        let existing_production = self.productions.insert(
            production.to_owned(),
            Entry {
                location: location.to_owned(),
                references: IndexSet::new(),
                defined_in: VersionSet::empty(),
                used_in: VersionSet::empty(),
            },
        );

        assert!(existing_production.is_none());
    }

    pub fn add_version(&mut self, production: &str, version_set: &VersionSet) {
        let production = self.productions.get_mut(production).unwrap();
        production.defined_in = production.defined_in.union(version_set);
    }

    pub fn is_defined_over(&self, production: &str, version_set: &VersionSet) -> bool {
        let production = self.productions.get(production).unwrap();
        return version_set.difference(&production.defined_in).is_empty();
    }

    pub fn add_reference(
        &mut self,
        production_name: &str,
        version_set: &VersionSet,
        reference_name: &str,
    ) {
        let reference = self.productions.get_mut(reference_name).unwrap();
        reference.used_in = reference.used_in.union(version_set);

        let production = self.productions.get_mut(production_name).unwrap();
        production.references.insert(reference_name.to_owned());
    }

    pub fn validate_not_used(&self, language: &LanguageDefinitionRef, reporter: &mut Reporter) {
        let required_productions = language.required_productions();

        for production_name in language.productions.keys() {
            if required_productions.contains(production_name.as_str()) {
                continue;
            }

            let production = self.productions.get(production_name).unwrap();
            let not_used_in = production.defined_in.difference(&production.used_in);

            if !not_used_in.is_empty() {
                reporter.report(
                    &production.location,
                    Errors::VersionsNotUsed(production_name.to_owned(), not_used_in),
                );
            }
        }
    }

    pub fn validate_not_reachable(
        &self,
        language: &LanguageDefinitionRef,
        reporter: &mut Reporter,
    ) {
        let mut visited = IndexSet::new();

        let mut queue = language
            .required_productions()
            .into_iter()
            .collect::<Vec<_>>();

        while let Some(production_name) = queue.pop() {
            if !visited.insert(production_name.to_owned()) {
                continue;
            }

            let references = &self.productions.get(production_name).unwrap().references;
            for reference in references {
                if !visited.contains(reference) {
                    queue.push(&reference);
                }
            }
        }

        for production_name in language.productions.keys() {
            if !visited.contains(production_name) {
                let location = &self.productions.get(production_name).unwrap().location;
                reporter.report(location, Errors::NotReachable(production_name.to_owned()));
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Production '{0}' is not used in versions: {1}")]
    VersionsNotUsed(String, VersionSet),
    #[error("Production '{0}' is not reachable from other rules in the grammar.")]
    NotReachable(String),
}
