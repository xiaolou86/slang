use anyhow::Result;
use infra_utils::{commands::Command, github::GitHub};

pub fn setup_npm() -> Result<()> {
    return if GitHub::is_running_in_ci() {
        Command::new("npm").arg("install").flag("--ci").run()
    } else {
        Command::new("npm").arg("install").run()
    };
}
