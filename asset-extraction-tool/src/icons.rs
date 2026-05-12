use std::{error::Error, fmt::Display, fs, path::PathBuf, str::FromStr};

use crate::{ImageFormat, VERSION};

use anyhow::Result;
use boilmaster_re_exports::asset::{format::Format, service::Service as AssetService};
use clap::Parser;

#[derive(Debug, Clone, Copy)]
pub struct Resolution {
	index: u8,
}

impl Resolution {
	pub fn filename_suffix(&self) -> String {
		match self.index {
			0 => "".into(),
			1..=u8::MAX => format!("_{self}"),
		}
	}
}

impl Display for Resolution {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let index = self.index;
		match index {
			0 => write!(f, "lr"),
			_ => write!(f, "hr{index}"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct ParseResolutionError;

impl Display for ParseResolutionError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Failed to parse resolution")
	}
}

impl Error for ParseResolutionError {}

impl FromStr for Resolution {
	type Err = ParseResolutionError;

	fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
		match s {
			"" | "lr" => Ok(Self { index: 0 }),
			_s if let Some(("", num)) = s.split_once("hr") => Ok(Self {
				index: num.parse().map_err(|_| ParseResolutionError)?,
			}),
			_ => Err(ParseResolutionError),
		}
	}
}

#[derive(Debug, Parser)]
pub struct IconsArgs {
	/// Output image format
	#[arg(short, long, default_value_t = ImageFormat::Png)]
	format: ImageFormat,

	/// Resolution to export. 
	/// 
	/// Defaults to `hr1`, which is the highest resolution available as of patch 7.5. Use `lr` for the low resolution.
	#[arg(short, long, default_value_t = Resolution { index: 1 }, value_parser = Resolution::from_str)]
	resolution: Resolution,
}

pub fn extract_icons(
	asset_service: AssetService,
	args: IconsArgs,
	output_dir: &PathBuf,
) -> Result<()> {
	let format = Format::from(args.format);
	let resolution_suffix = args.resolution.filename_suffix();

	for id in 0..1000000 {
		let folder_id = id - (id % 1000);
		let path = format!("ui/icon/{folder_id:0>6}/{id:0>6}{resolution_suffix}.tex");

		let bytes = match asset_service.convert(VERSION, &path, format) {
			Err(boilmaster_re_exports::asset::error::Error::NotFound(_path)) => continue,
			err => err?,
		};

		let output_path = output_dir.join(path);
		if let Some(parent) = output_path.parent() {
			fs::create_dir_all(parent)?;
		}
		fs::write(output_path.with_extension(format.extension()), bytes)?;
	}

	Ok(())
}
