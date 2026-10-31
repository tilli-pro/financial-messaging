# iso-8583

A spec-driven ISO 8583 codec for Rust.

> **Status: pre-release.** Only the message type indicator (`Mti`) is
> implemented. Nothing is published to crates.io.

## Scope

- Message type indicator (MTI).
- Primary, secondary and tertiary bitmaps.
- Data elements, described by per-host **field tables** (JSON).
- Encodings: ASCII, EBCDIC, packed BCD and binary, for both lengths and
  values.
- Composite fields (subfields) and TLV data (for example EMV in DE 55).
- Non-blocking validation: decoding reports every problem it finds alongside
  the decoded message instead of stopping at the first one.
- Generic ISO 8583:1987, 1993 and 2003 field tables, shipped with the crate.

## Non-goals

- Transport, framing, length headers and session management.
- MAC computation, PIN blocks and key management.
- Network-specific field tables (Visa, Mastercard, acquirer hosts). Their
  specifications are confidential; users load their own tables as data.

These belong to the connector that uses this crate.

## Field tables

Every host interprets ISO 8583 differently, so the layout of each data
element is data, not code. A field table is a JSON document; one data element
definition looks like this:

```json
{
  "number": 2,
  "name": "Primary account number",
  "kind": "numeric",
  "length": { "type": "llvar", "max": 19, "prefix_encoding": "ascii" },
  "encoding": "ascii"
}
```

The schema of a field table is not final and will change before the first
release.

## Example

```rust
use iso_8583::Mti;

let mti = Mti::parse("0200")?;
assert_eq!((mti.version(), mti.class(), mti.function(), mti.origin()), (0, 2, 0, 0));
```

## License

MIT. See [LICENSE](../../LICENSE).
