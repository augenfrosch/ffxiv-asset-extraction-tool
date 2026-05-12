#[rustfmt::skip]
#[path = "../boilmaster/crates/bm_asset/src/error.rs"]
pub mod error;

#[rustfmt::skip]
#[path = "../boilmaster/crates/bm_asset/src/format.rs"]
pub mod format;

#[rustfmt::skip]
#[path = "../boilmaster/crates/bm_asset/src/texture.rs"]
pub mod texture;

mod convert {
	use crate::asset::format;

	pub trait Converter {}

	pub struct Image;

	impl Converter for Image {}

	#[expect(dead_code)]
	fn use_unused_to_avoid_having_to_do_even_more_cursed_things() {
		let _ = format::Format::Png.converter();
	}
}
