# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the crates follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Nothing unreleased yet.

## [0.0.1] - 2026-10-31

Pre-release: crate names and workspace established; `Mti` parser in iso-8583;
`MessageIdentifier` in iso-20022; codegen CLI stub.

### Added

- Workspace scaffold: `iso-8583`, `iso-20022` and `iso-20022-codegen` crates,
  CI, licensing and project policies.
- `iso-8583`: `Mti` message type indicator parsing.
- `iso-20022`: `MessageIdentifier` parsing (`area.functionality.variant.version`).

[Unreleased]: https://github.com/tilli-pro/financial-messaging/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/tilli-pro/financial-messaging/releases/tag/v0.0.1
