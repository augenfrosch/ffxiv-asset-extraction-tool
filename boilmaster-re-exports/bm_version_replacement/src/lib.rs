use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct VersionKey;

impl Display for VersionKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Installed Version")
	}
}
