mod cargo;
mod changesets;
mod github_release;
mod lock_files;
mod npm;

use anyhow::Result;
use clap::{Parser, ValueEnum};

use crate::{
    commands::publish::{
        cargo::publish_cargo, changesets::publish_changesets,
        github_release::publish_github_release, lock_files::publish_lock_files, npm::publish_npm,
    },
    utils::{ClapExtensions, Terminal},
};

#[derive(Clone, Debug, Parser)]
pub struct PublishController {
    command: PublishCommand,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum PublishCommand {
    /// Consume pending changesets, update changelogs and package versions, then send a PR.
    Changesets,
    /// Publish source packages to [npmjs.com].
    Npm,
    /// Publish source crates to [crates.io].
    Cargo,
    /// Publish a new release in the GitHub repository.
    GithubRelease,
    /// Publish updated lock files to a PR for review.
    LockFiles,
}

impl PublishController {
    pub fn execute(&self) -> Result<()> {
        Terminal::step(format!("publish {name}", name = self.command.clap_name()));

        return match self.command {
            PublishCommand::Changesets => publish_changesets(),
            PublishCommand::Npm => publish_npm(),
            PublishCommand::Cargo => publish_cargo(),
            PublishCommand::GithubRelease => publish_github_release(),
            PublishCommand::LockFiles => publish_lock_files(),
        };
    }
}
