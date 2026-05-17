#[rustfmt::skip]
#[path = "../boilmaster/crates/bm_asset/src/convert.rs"]
mod convert;

#[rustfmt::skip]
#[path = "../boilmaster/crates/bm_asset/src/error.rs"]
pub mod error;

#[rustfmt::skip]
#[path = "../boilmaster/crates/bm_asset/src/format.rs"]
pub mod format;

#[expect(clippy::needless_borrow)]
#[rustfmt::skip]
#[path = "../boilmaster/crates/bm_asset/src/service.rs"]
pub mod service;

// MAYBE: I should just copy the files into this crate directly
// That would also allow me to adapt them if(/when?) `scree` gets asset reads
#[expect(clippy::into_iter_on_ref)]
#[rustfmt::skip]
#[path = "../boilmaster/crates/bm_asset/src/texture.rs"]
mod texture;
