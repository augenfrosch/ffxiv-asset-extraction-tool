use std::{fs, path::PathBuf, sync::Arc};

use anyhow::{Context, Result};
use bm_data::Data;
use bm_version::VersionKey;
use boilmaster_re_exports::asset::{format::Format, service::Service as AssetService};
use clap::Parser;
use ironworks::{
	Ironworks,
	sqpack::{Install, SqPack},
};

#[derive(Debug, Parser)]
struct Args {
	/// Set the input directory containing the game install.
	///
	/// The directory must be the base directory for the game install, i.e., it must contain the `game` subdirectory.
	#[arg(short, long = "input", value_name = "PATH")]
	input_dir: PathBuf,

	/// Set the output directory.
	///
	/// Directories are created if they are missing. The default creates a new `output` directory in the current working directory.
	#[arg(short, long = "output", value_name = "PATH", default_value = "output")]
	output_dir: PathBuf,
}

fn main() -> Result<()> {
	let Args {
		input_dir,
		output_dir,
	} = Args::parse();

	fs::create_dir_all(&output_dir)?;

	let ironworks = Ironworks::new().with_resource(SqPack::new(Install::at(&input_dir)));
	let data = Arc::new(Data {
		ironworks: Arc::new(ironworks),
	});

	let asset_service = AssetService::new(data);
	const VERSION: VersionKey = VersionKey;

	let path = "ui/icon/065000/065002_hr1.tex";
	let format = Format::Png;
	let bytes = asset_service.convert(VERSION, path, format)?;

	let output_path = output_dir.join(path);
	if let Some(parent) = output_path.parent() {
		fs::create_dir_all(parent)?;
	}
	fs::write(output_path.with_extension(format.extension()), bytes)?;

	Ok(())
}
