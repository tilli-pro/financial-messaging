//! Command-line entry point for `iso-20022-codegen`.

use std::process::ExitCode;

const USAGE: &str = "\
iso-20022-codegen: generate typed Rust from ISO 20022 XSDs

USAGE:
    iso-20022-codegen --xsd <FILE.xsd> --out <DIR>

OPTIONS:
    --xsd <FILE>    ISO 20022 message schema, e.g. pacs.008.001.08.xsd
    --out <DIR>     Directory to write the generated Rust module into

STATUS:
    Not implemented yet. This binary only prints this message.
";

fn main() -> ExitCode {
    eprint!("{USAGE}");
    ExitCode::from(2)
}
