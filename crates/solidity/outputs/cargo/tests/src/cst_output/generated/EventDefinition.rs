// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn transfer() -> Result<()> {
    return run("EventDefinition", "transfer");
}
