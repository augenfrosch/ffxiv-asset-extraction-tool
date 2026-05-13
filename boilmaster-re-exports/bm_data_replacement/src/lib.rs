use std::sync::Arc;

use anyhow::Result;
use bm_version::VersionKey;
use ironworks::Ironworks;

pub struct Data {
	pub ironworks: Arc<Ironworks>,
}

pub struct Version {
	ironworks: Arc<Ironworks>,
}

impl Data {
	#[expect(clippy::missing_errors_doc)]
	pub fn version(&self, _version: VersionKey) -> Result<Version> {
		Ok(Version {
			ironworks: self.ironworks.clone(),
		})
	}
}

impl Version {
	#[must_use]
	pub fn ironworks(&self) -> Arc<Ironworks> {
		self.ironworks.clone()
	}
}
