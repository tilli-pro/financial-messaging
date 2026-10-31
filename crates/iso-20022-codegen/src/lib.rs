//! Generator that turns official ISO 20022 XML Schemas (XSDs) into strongly
//! typed Rust for the `iso-20022` crate.
//!
//! The pipeline has three stages:
//!
//! 1. **Parse** the XSD into a schema model.
//! 2. **Lower** the schema into an intermediate representation ([`ir`]) where
//!    code lists, facets and cardinalities are explicit.
//! 3. **Emit** Rust source from the IR: enums for code lists, validated
//!    newtypes for facets, typed containers for cardinalities, with `serde`
//!    attributes for `quick-xml`.
//!
//! # Status
//!
//! Pre-release. None of the stages are implemented yet.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// The intermediate representation between a parsed XSD and emitted Rust.
///
/// Placeholder: the IR is not designed yet.
pub mod ir {}

#[cfg(test)]
mod tests {
    #[test]
    fn ir_module_exists() {
        // Compiles only while `crate::ir` exists.
        #[allow(unused_imports)]
        use crate::ir as _;
    }
}
