use std::{
	error::Error,
	fmt::{Debug, Display},
	fs,
	num::ParseIntError,
	ops::{Range as OpsRange, RangeInclusive},
	path::Path,
	str::FromStr,
};

use crate::{ImageFormat, VERSION};

use anyhow::Result;
use boilmaster_re_exports::asset::{format::Format, service::Service as AssetService};
use clap::Parser;

#[derive(Debug, Clone, Copy)]
pub struct Resolution {
	index: u8,
}

impl Resolution {
	pub fn filename_suffix(self) -> String {
		match self.index {
			0 => String::new(),
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

#[derive(Debug, Clone)]
pub enum IconIdRange {
	Inclusive(RangeInclusive<u32>),
	Exclusive(OpsRange<u32>),
}

impl From<RangeInclusive<u32>> for IconIdRange {
	fn from(range: RangeInclusive<u32>) -> Self {
		Self::Inclusive(range)
	}
}

impl From<OpsRange<u32>> for IconIdRange {
	fn from(range: OpsRange<u32>) -> Self {
		Self::Exclusive(range)
	}
}

impl Display for IconIdRange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let (start, mark, end) = match self {
			IconIdRange::Inclusive(range_inclusive) => {
				(range_inclusive.start(), "..=", range_inclusive.end())
			},
			IconIdRange::Exclusive(range) => (&range.start, "..", &range.end),
		};
		write!(f, "{start:0>6}{mark}{end:0>6}")
	}
}

impl IntoIterator for IconIdRange {
	type Item = u32;

	// TODO: look into finding a way to avoid this. The code was a lot messier before,
	// but it might be easier to just implement a completely custom range and iterator type
	type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

	fn into_iter(self) -> Self::IntoIter {
		match self {
			IconIdRange::Inclusive(range_inclusive) => Box::new(range_inclusive),
			IconIdRange::Exclusive(range) => Box::new(range),
		}
	}
}

#[derive(Debug, Clone)]
pub enum ParseRangeError {
	MissingMark,
	InvalidMark,
	BoundNotParsable(ParseIntError),
}

impl Display for ParseRangeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::MissingMark | Self::InvalidMark => {
				write!(f, "Range must contain either `..` or `..=`")
			},
			Self::BoundNotParsable(err) => write!(f, "Failed to parse range delimiter: {err}"),
		}
	}
}

impl Error for ParseRangeError {}

impl FromStr for IconIdRange {
	type Err = ParseRangeError;

	fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
		let (start, rest) = s.split_once("..").ok_or(ParseRangeError::MissingMark)?;
		let (inclusive, end) = match rest.split_once('=') {
			Some(("", end)) => (true, end),
			Some((_, _end)) => return Err(ParseRangeError::InvalidMark),
			None => (false, rest),
		};

		let start = u32::from_str(start).map_err(ParseRangeError::BoundNotParsable)?;
		let end = u32::from_str(end).map_err(ParseRangeError::BoundNotParsable)?;

		if inclusive {
			Ok(Self::Inclusive(RangeInclusive::new(start, end)))
		} else {
			Ok(Self::Exclusive(OpsRange { start, end }))
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
	#[arg(long, default_value_t = Resolution { index: 1 }, value_parser = Resolution::from_str)]
	resolution: Resolution,

	/// Range of icon IDs to attempt to export.
	///
	/// IDs that don't map to non-existant icons are ignored. The ranges are specified using the Rust syntax for exclusive and inclusive ranges, e.g., `020000..058000` and `027104..=027110`.
	#[arg{long, default_value_t = IconIdRange::from(0..1000000)}]
	range: IconIdRange,
}

pub fn extract_icons(
	asset_service: &AssetService,
	args: IconsArgs,
	output_dir: &Path,
) -> Result<()> {
	let format = Format::from(args.format);
	let resolution_suffix = args.resolution.filename_suffix();

	for id in args.range {
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
