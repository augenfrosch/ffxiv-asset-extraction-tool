use std::{
	collections::HashSet,
	fs,
	path::Path,
	sync::{Arc, LazyLock},
};

use crate::{ImageFormat, VERSION};

use anyhow::Result;
use boilmaster_re_exports::asset::{format::Format, service::Service as AssetService};
use clap::Parser;
use ironworks::{
	Ironworks,
	excel::{ColumnSpecifier, Excel},
	file::exh::{ColumnDefinition, ColumnKind},
	sestring::{
		SeString,
		format::{Input, PlainString, format},
	},
};

#[derive(Debug, Parser)]
pub struct MapsArgs {
	/// Output image format
	#[arg(short, long, default_value_t = ImageFormat::Png)]
	format: ImageFormat,
}

static INPUT: LazyLock<Input> = LazyLock::new(Input::new);

pub fn se_string_to_plaintext(se_string: &SeString) -> Result<String> {
	let mut writer = PlainString::new();
	format(se_string.as_ref(), &INPUT, &mut writer)?;
	Ok(String::from(writer))
}

// TODO: look into some oddities, e.g., `r2t2/00` is completely black, `s1i3/01` and many other maps have black borders
pub fn extract_maps(
	ironworks: Arc<Ironworks>,
	asset_service: &AssetService,
	args: &MapsArgs,
	output_dir: &Path,
) -> Result<()> {
	let format = Format::from(args.format);
	let excel = Excel::new(ironworks);
	let map_sheet = excel.sheet("Map")?;

	let output_dir = output_dir.join("maps");
	fs::create_dir_all(&output_dir)?;

	let mut seen = HashSet::new();
	for map_row in map_sheet {
		let map_row = map_row?;

		// TODO: make the offset/`ColumnSpecifier` configurable in case the layout/schema changes
		let id_string = match map_row.field(ColumnSpecifier::Definition(&ColumnDefinition {
			kind: ColumnKind::String,
			offset: 0x00,
		}))? {
			ironworks::excel::Field::String(se_string) => se_string_to_plaintext(&se_string)?,
			_ => unimplemented!(
				"Non-string field encountered when attempting to access the `Map` sheet's `Id` field. Offset has likely changed."
			),
		};
		if seen.contains(&id_string) {
			continue;
		}
		let Some((territory, index)) = id_string.split_once('/') else {
			continue;
		};

		let bytes = asset_service.map(VERSION, territory, index)?;

		let output_path = output_dir.join(format!("{territory}-{index}"));
		if let Some(parent) = output_path.parent() {
			fs::create_dir_all(parent)?;
		}
		fs::write(output_path.with_extension(format.extension()), bytes)?;

		seen.insert(id_string);
	}

	Ok(())
}
