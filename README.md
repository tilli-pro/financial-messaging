# financial-messaging

[![CI](https://github.com/tilli-pro/financial-messaging/actions/workflows/ci.yml/badge.svg)](https://github.com/tilli-pro/financial-messaging/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Rust codecs for the two message standards that move money between banks and
card networks: **ISO 8583** (card authorization and clearing) and
**ISO 20022** (MX payments, clearing and cash management). The goal is a pair
of small, dependency-light libraries whose types make malformed messages hard
to construct and whose parsers are proven not to panic on hostile input.

> **Status: pre-release.** The workspace, the method and the first types are
> in place; the codecs themselves are not written yet. Nothing is published
> to crates.io. Do not depend on this in production.

## Crates

| Crate | Purpose | Status | Docs |
| --- | --- | --- | --- |
| [`iso-8583`](crates/iso-8583) | Spec-driven ISO 8583 codec: MTI, bitmaps, data elements, per-host field tables | Pre-release, not published | ![docs.rs](https://img.shields.io/badge/docs.rs-unpublished-lightgrey) |
| [`iso-20022`](crates/iso-20022) | Strongly typed ISO 20022 (MX) messages generated from the official XSDs | Pre-release, not published | ![docs.rs](https://img.shields.io/badge/docs.rs-unpublished-lightgrey) |
| [`iso-20022-codegen`](crates/iso-20022-codegen) | The XSD to Rust generator behind `iso-20022` | Pre-release, not published | ![docs.rs](https://img.shields.io/badge/docs.rs-unpublished-lightgrey) |

## How we verify

Payment messages fail expensively, so every change is held to the same
ladder. Each rung catches what the one before it cannot.

1. **Correct by construction.** Types encode the standard: code lists are
   enums, facets are validated newtypes, cardinalities are typed containers,
   ISO 8583 fields are described by an explicit field table. Invalid states
   should not be representable.
2. **Property tests.** [`proptest`](https://crates.io/crates/proptest) checks
   that `decode(encode(x)) == x` for generated values of every type, and that
   decoders reject or report (never crash on) arbitrary bytes.
3. **Differential tests.** Generated ISO 20022 messages are checked against an
   independent XSD validator, and published sample messages are decoded,
   re-encoded and compared.
4. **Proofs.** [Kani](https://github.com/model-checking/kani) harnesses prove
   that the parsers cannot panic and never read out of bounds, for all inputs
   up to a bound.
5. **Certification.** Scheme and partner certification is the final gate.
   It happens in the products that use these crates, not in this repository.

### Where test vectors come from

Neither standard ships a conformance suite, so we are explicit about sources:

- **ISO 20022:** the example instances in the Message Definition Reports
  (MDRs) published in the [ISO 20022 catalogue](https://www.iso20022.org/iso-20022-message-definitions),
  and public sample sets published by schemes and market infrastructures.
  Every fixture records its source URL.
- **ISO 8583:** there are no public vectors. Real test data only comes from
  network and acquirer certification, which is confidential and never enters
  this repository. Tests here use synthetic messages built from the generic
  1987/1993/2003 field tables.

## What this is not

- **Not a transport.** No TCP framing, length headers, TLS, MQ, SFTP or
  session management.
- **Not a security module.** No MAC computation, PIN blocks, key management or
  HSM integration.
- **Not partner profiles.** No Visa, Mastercard, acquirer or bank-specific
  field tables or usage guidelines. Those are loaded as data by users.
- **Not certification.** Passing these tests does not certify anything with
  any scheme or partner.

## Roadmap

- `iso-8583`: bitmaps (primary, secondary, tertiary), data elements, JSON
  field tables, ASCII / EBCDIC / packed BCD / binary encodings, composite
  fields and TLV, non-blocking validation, generic 1987/1993/2003 tables.
- `iso-20022-codegen`: XSD parser, intermediate representation, Rust emitter.
- `iso-20022`: the first generated message set, at the versions our first
  adopters need:
  `pain.001`, `pain.002`, `pacs.008`, `pacs.002`, `pacs.004`, `camt.053`,
  `camt.054`, `camt.056`. Further messages are generated on request.

## Rust version

The minimum supported Rust version (MSRV) is **1.94**. The policy is
*latest stable minus two*: the MSRV may be raised in any minor release, never
beyond two releases behind the current stable. Development uses the toolchain
pinned in [`rust-toolchain.toml`](rust-toolchain.toml).

## Contributing and security

Contributions are welcome; read [CONTRIBUTING.md](CONTRIBUTING.md) first,
especially the rule on partner-confidential material. Report vulnerabilities
privately as described in [SECURITY.md](SECURITY.md), never in a public issue.
Participation is governed by our [Code of Conduct](CODE_OF_CONDUCT.md).

## License

Licensed under the [MIT License](LICENSE).

---

Maintained by [Tilli Software](https://github.com/tilli-pro); it powers our
payments platform.
