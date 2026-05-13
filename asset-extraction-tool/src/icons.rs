use core::range::{RangeInclusive, RangeInclusiveIter};
use std::{
	error::Error,
	fmt::{Debug, Display},
	fs,
	num::ParseIntError,
	ops::{Range as OpsRange, RangeInclusive as OpsRangeInclusive},
	path::Path,
	str::FromStr,
};

use crate::{ImageFormat, VERSION};

use anyhow::Result;
use boilmaster_re_exports::asset::{format::Format, service::Service as AssetService};
use clap::{ArgAction, Parser};

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
pub struct IconIdRange {
	start: u32,
	end: u32,
	inclusive: bool,
}

impl IconIdRange {
	pub fn into_inclusive(self) -> Self {
		if self.inclusive {
			self
		} else {
			match self.end.checked_sub(1) {
				Some(end_inclusive) => Self {
					start: self.start,
					end: end_inclusive,
					inclusive: true,
				},
				// This would look awkward when displayed but is currently only used where the result isn't shown to the user. MAYBE: look into this later
				None => Self {
					start: u32::MAX,
					end: u32::MIN,
					inclusive: true,
				},
			}
		}
	}
}

impl From<OpsRangeInclusive<u32>> for IconIdRange {
	fn from(range: OpsRangeInclusive<u32>) -> Self {
		Self {
			start: *range.start(),
			end: *range.end(),
			inclusive: true,
		}
	}
}

impl From<OpsRange<u32>> for IconIdRange {
	fn from(range: OpsRange<u32>) -> Self {
		Self {
			start: range.start,
			end: range.end,
			inclusive: false,
		}
	}
}

impl From<IconIdRange> for RangeInclusive<u32> {
	fn from(range: IconIdRange) -> Self {
		let IconIdRange { start, end, .. } = range.into_inclusive();
		RangeInclusive {
			start: start,
			last: end,
		}
	}
}

impl Display for IconIdRange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let Self {
			start,
			end,
			inclusive,
		} = self;
		let mark = if *inclusive { "..=" } else { ".." };
		write!(f, "{start:0>6}{mark}{end:0>6}")
	}
}

struct IconFolderIdRange {
	start_folder_id: u32,
	end_folder_id: u32,
}

impl IconFolderIdRange {
	pub fn id_range(folder_id: u32) -> RangeInclusive<u32> {
		RangeInclusive { start: folder_id, last: folder_id.saturating_add(999) }
	}
}

impl From<IconIdRange> for IconFolderIdRange {
	fn from(range: IconIdRange) -> Self {
		let IconIdRange { start, end, .. } = range.into_inclusive();
		Self {
			start_folder_id: start - start % 1000,
			end_folder_id: end - end % 1000,
		}
	}
}

impl Iterator for IconFolderIdRange {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		let Self { start_folder_id, end_folder_id } = self;
		if start_folder_id <= end_folder_id {
			let next = *start_folder_id;
			// It is guaranteed that the iteration doesn't get "stuck" at the `end_folder_id` if it is a multiple of 1000, since `u32::MAX` is not a multiple of 1000
			*start_folder_id = start_folder_id.saturating_add(1000);
			Some(next)
		} else {
			None
		}
	}
}

impl IntoIterator for IconIdRange {
	type Item = u32;

	type IntoIter = RangeInclusiveIter<u32>;

	fn into_iter(self) -> Self::IntoIter {
		RangeInclusive::from(self).into_iter()
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

		Ok(Self {
			start,
			end,
			inclusive,
		})
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

	/// Export HQ icons in addition to the normal, NQ icons.
	///
	/// This is only applicable to item based icons.
	#[arg(long, default_value_t = true, action = ArgAction::Set)]
	hq: bool,

	/// Export icons for the specified languages if they exist.
	///
	/// Passing an empty list effetively skips the export of localized icons.
	#[arg(long, num_args = 0.., default_values = ["ja", "en", "de", "fr", "chs", "ko", "tc"], value_name = "LANGUAGE")]
	languages: Vec<String>,
}

pub fn extract_icons(
	asset_service: &AssetService,
	args: IconsArgs,
	output_dir: &Path,
) -> Result<()> {
	let _span = tracing::debug_span!("Extract icons").entered();

	let format = Format::from(args.format);
	let mut subfolders = args.languages;
	subfolders.insert(0, String::new());
	if args.hq {
		subfolders.insert(1, "hq".to_string());
	}
	let resolution_suffix = args.resolution.filename_suffix();

	// let id_range = RangeInclusive::from(args.range);
	// let folder_range = IconFolderIdRange::from(args.range.clone());
	// for folder_id in folder_range {
	// 	let folder_id_range = IconFolderIdRange::id_range(folder_id);

	// 	// TODO check if folder exists
	// 	for id in args
	// }
	for id in args.range {
		let folder_id = id - (id % 1000);
		for subfolder in &subfolders {
			let subfolder_separator = if subfolder.is_empty() { "" } else { "/" };
			let path = format!(
				"ui/icon/{folder_id:0>6}/{subfolder}{subfolder_separator}{id:0>6}{resolution_suffix}.tex"
			);

			let bytes = match asset_service.convert(VERSION, &path, format) {
				Ok(bytes) => bytes,
				Err(boilmaster_re_exports::asset::error::Error::NotFound(_path)) => continue,
				Err(boilmaster_re_exports::asset::error::Error::Failure(err)) => {
					tracing::debug!(path, "Failure to read asset: {err}", err = err.root_cause());
					continue;
				},
				Err(err) => Err(err)?,
			};

			let output_path = output_dir.join(path);
			if let Some(parent) = output_path.parent() {
				fs::create_dir_all(parent)?;
			}
			fs::write(output_path.with_extension(format.extension()), bytes)?;
		}
	}

	Ok(())
}
