// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn identifier_path() -> Result<()> {
    return run("UsingDirectiveSymbol", "identifier_path");
}

#[test]
fn identifier_path_as_operator() -> Result<()> {
    return run("UsingDirectiveSymbol", "identifier_path_as_operator");
}

#[test]
fn single_id() -> Result<()> {
    return run("UsingDirectiveSymbol", "single_id");
}

#[test]
fn single_id_as_operator() -> Result<()> {
    return run("UsingDirectiveSymbol", "single_id_as_operator");
}
