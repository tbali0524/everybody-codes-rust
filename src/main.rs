//! # Everybode Codes solutions CLI runner binary crate.

#![deny(broken_intra_doc_links)]

use ec::ec::cli::run;
use std::process::ExitCode;

pub fn main() -> ExitCode {
    run()
}
