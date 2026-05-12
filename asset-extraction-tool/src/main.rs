use std::path::PathBuf;

use anyhow::Result;
use boilmaster_re_exports::asset::texture::{read, write};
use clap::Parser;
use ironworks::{Ironworks, sqpack::{Install, SqPack}};

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
	let Args { input_dir, output_dir } = Args::parse();

    let ironworks = Ironworks::new().with_resource(SqPack::new(Install::at(&input_dir)));

	dbg!(input_dir, output_dir);

	Ok(())
}
