#![deny(unsafe_code)]
#![deny(missing_docs)]
//! Low-level loader plugins for the SIM bootloader.
//!
//! The crate owns loader mechanisms that the `sim` binary composes behind
//! feature gates. It deliberately depends on kernel and codec contracts rather
//! than the SDK umbrella.
//!
//! Native and wasm loaders both surface `site` exports as opaque registry
//! values keyed by placement symbols. The kernel stores the value and export
//! record; server and agent libraries give the value `EvalSite` behavior.

#[cfg(all(feature = "dynamic-native", not(target_arch = "wasm32")))]
mod manifest;
#[cfg(all(feature = "dynamic-native", not(target_arch = "wasm32")))]
mod native;
#[cfg(all(feature = "dynamic-native", not(target_arch = "wasm32")))]
mod native_class;
#[cfg(all(feature = "dynamic-native", not(target_arch = "wasm32")))]
mod native_macro;
#[cfg(all(feature = "dynamic-native", not(target_arch = "wasm32")))]
mod native_number;
#[cfg(all(feature = "dynamic-native", not(target_arch = "wasm32")))]
mod native_site;
#[cfg(all(feature = "dynamic-native", not(target_arch = "wasm32")))]
mod shape;
#[cfg(all(feature = "dynamic-native", not(target_arch = "wasm32")))]
mod shared;
mod source;
#[cfg(any(feature = "wasm", test))]
mod wasm;

#[cfg(all(feature = "dynamic-native", not(target_arch = "wasm32")))]
pub use native::{
    NativeDylibLoader, NativeGuest, encode_native_manifest_response, validate_native_abi_header,
};
#[cfg(all(feature = "dynamic-native", not(target_arch = "wasm32")))]
pub use native_macro::NativeAbiMacro;
pub use source::{
    BYTES_SOURCE_KIND, CONTENT_ADDRESS_SOURCE_KIND, PATH_SOURCE_KIND, URL_SOURCE_KIND,
    bytes_from_payload, bytes_from_source, bytes_source, bytes_source_kind, bytes_source_spec,
    catalog_bytes_source, catalog_content_address_source, catalog_path_source, catalog_url_source,
    content_address_payload, content_address_source, content_address_source_kind,
    content_address_source_spec, is_bytes_source, is_path_source, is_url_source, path_from_payload,
    path_from_source, path_payload, path_source, path_source_kind, path_source_spec,
    url_from_payload, url_from_source, url_source, url_source_kind, url_source_spec,
};
#[cfg(any(feature = "wasm", test))]
pub use wasm::{WasmLoader, wasm_load_capability};
