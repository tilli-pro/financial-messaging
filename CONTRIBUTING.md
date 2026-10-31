# Contributing

Thank you for your interest in `financial-messaging`. This project carries
payment messages, so the bar for changes is high and the process is explicit.

## Never contribute confidential material

**If it came from an NDA'd interface specification, it does not belong here.**

That includes card network and acquirer host specifications, bank or
market-infrastructure implementation guides that are not public, certification
test scripts and their messages, and any real cardholder, account or
transaction data. Network- and partner-specific field tables are loaded by
users as data; they are never committed to this repository. Pull requests that
contain such material will be closed and the history rewritten.

Public sources are welcome: the ISO 20022 catalogue (XSDs, MDRs and their
example instances) and samples that schemes publish openly. Record the source
URL next to every fixture.

## Developer Certificate of Origin

Every commit must be signed off under the
[Developer Certificate of Origin](https://developercertificate.org/), certifying
that you wrote the change or otherwise have the right to submit it under the
MIT license:

```sh
git commit -s
```

This adds a `Signed-off-by: Your Name <you@example.com>` trailer matching your
commit author.

## Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/):
`feat(iso-8583): decode secondary bitmap`, `fix(iso-20022): ...`,
`docs: ...`, `test: ...`, `chore: ...`. Scope is the crate name when the
change touches one crate.

## Before you open a pull request

All of these must pass; CI runs the same commands.

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo deny check   # if you have cargo-deny installed
```

New behavior needs tests, including a property test where there is an
encode/decode pair. Library code may not use `unwrap`, `expect`, `panic!` or
`unsafe`; return an error instead.

## Adding an ISO 20022 message version

1. Open an issue naming the message and version (for example
   `pacs.008.001.08`) and who needs it.
2. Download the XSD and the MDR example instances from the
   [ISO 20022 catalogue](https://www.iso20022.org/iso-20022-message-definitions).
   Do not commit the XSD except as a test fixture with its source URL.
3. Generate the module with `iso-20022-codegen` and commit the generated code
   together with the command used to produce it. Do not hand-edit generated
   code; fix the generator instead.
4. Add round-trip, differential (independent XSD validator) and sample-message
   tests.

## Adding an ISO 8583 field table

Only generic tables derived from the ISO 8583:1987, 1993 or 2003 standards
are accepted here. Write the table as JSON, cite the standard clause for each
data element, and add tests that encode and decode a synthetic message using
every element in the table. Network and acquirer tables stay with their users
(see the first section).

## Reporting bugs and vulnerabilities

Use GitHub issues for bugs. Report security issues privately as described in
[SECURITY.md](SECURITY.md).

## Conduct

This project follows the [Code of Conduct](CODE_OF_CONDUCT.md).
