use std::{fmt::Display, fs, path::PathBuf, sync::Arc};

mod icons;
use icons::{IconsArgs, extract_icons};

use anyhow::Result;
use bm_data::Data;
use bm_version::VersionKey;
use boilmaster_re_exports::asset::{format::Format, service::Service as AssetService};
use clap::{Parser, Subcommand, ValueEnum};
use ironworks::{
	Ironworks,
	sqpack::{Install, SqPack},
};

const VERSION: VersionKey = VersionKey; // Stand-in version key for the installed version

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
	#[command(subcommand)]
	command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
	// /// Export assets
	// Assets(AssetsArgs),
	/// Export all icons
	Icons(IconsArgs),
	/// Export all maps
	Maps(MapsArgs),
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ImageFormat {
	Jpg,
	Png,
	Webp,
}

impl Display for ImageFormat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				ImageFormat::Jpg => "jpg",
				ImageFormat::Png => "png",
				ImageFormat::Webp => "webp",
			}
		)
	}
}

impl From<ImageFormat> for Format {
	fn from(value: ImageFormat) -> Self {
		match value {
			ImageFormat::Jpg => Self::Jpeg,
			ImageFormat::Png => Self::Png,
			ImageFormat::Webp => Self::Webp,
		}
	}
}

// #[derive(Debug, Parser)]
// struct AssetsArgs {
// 	#[arg(short, long, value_name = "ASSET_PATH")]
// 	path: String,
// 	#[arg(short, long)]
// 	convert: bool,
// 	#[arg(long, required_if_eq("convert", "true"))]
// 	image_format: Option<ImageFormat>,
// }

#[derive(Debug, Parser)]
pub struct MapsArgs {
	#[arg(short, long, default_value_t = ImageFormat::Png)]
	format: ImageFormat,
}

fn extract_maps(asset_service: AssetService, args: MapsArgs, output_dir: &PathBuf) -> Result<()> {
	Ok(())
}

fn main() -> Result<()> {
	let Args {
		input_dir,
		output_dir,
		command,
	} = Args::parse();

	fs::create_dir_all(&output_dir)?;

	let ironworks = Ironworks::new().with_resource(SqPack::new(Install::at(&input_dir)));
	let data = Arc::new(Data {
		ironworks: Arc::new(ironworks),
	});
	let asset_service = AssetService::new(data);

	match command {
		Command::Icons(args) => extract_icons(asset_service, args, &output_dir)?,
		Command::Maps(args) => extract_maps(asset_service, args, &output_dir)?,
	}

	Ok(())
}
