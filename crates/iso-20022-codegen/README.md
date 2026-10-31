# iso-20022-codegen

Generates the strongly typed Rust in [`iso-20022`](../iso-20022) from the
official ISO 20022 XML Schemas.

> **Status: pre-release.** The CLI prints usage and exits; none of the
> pipeline is implemented. Nothing is published to crates.io.

## Pipeline

```text
XSD  ──parse──▶  schema model  ──lower──▶  IR  ──emit──▶  Rust source
```

1. **Parse.** Read the message XSD (for example `pacs.008.001.08.xsd`) into a
   schema model: complex types, simple types, facets, enumerations,
   sequences, choices, occurrence bounds.
2. **Lower.** Translate the schema into an intermediate representation in
   which every constraint is explicit: code lists, facets (length, pattern,
   digits, ranges) and cardinalities.
3. **Emit.** Generate Rust: enums for code lists, validated newtypes for
   facets, typed containers for cardinalities, and `serde` attributes for
   `quick-xml`. Output is deterministic and formatted, so regenerating
   produces no diff unless the schema changed.

## Usage (planned)

```sh
iso-20022-codegen --xsd pacs.008.001.08.xsd --out src/generated/
```

Schemas come from the
[ISO 20022 message catalogue](https://www.iso20022.org/iso-20022-message-definitions)
and are not committed to this repository except as test fixtures with their
source URL.

## License

MIT. See [LICENSE](../../LICENSE).
