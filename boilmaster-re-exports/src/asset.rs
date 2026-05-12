// Best I could come up with without having to manually ensure everything in the modules is used in some form
#![allow(dead_code)]

#[rustfmt::skip]
#[path = "../boilmaster/crates/bm_asset/src/convert.rs"]
mod convert;

#[rustfmt::skip]
#[path = "../boilmaster/crates/bm_asset/src/error.rs"]
pub mod error;

#[rustfmt::skip]
#[path = "../boilmaster/crates/bm_asset/src/format.rs"]
pub mod format;

#[rustfmt::skip]
#[path = "../boilmaster/crates/bm_asset/src/service.rs"]
pub mod service;

#[rustfmt::skip]
#[path = "../boilmaster/crates/bm_asset/src/texture.rs"]
mod texture;
