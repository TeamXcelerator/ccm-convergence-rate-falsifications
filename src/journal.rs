use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Pass,
    Fail,
    Incomplete,
    Unassessed,
    Unsupported,
}
impl Status {
    #[cfg(feature = "hp")]
    pub fn label(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::Incomplete => "INCOMPLETE",
            Self::Unassessed => "UNASSESSED",
            Self::Unsupported => "UNSUPPORTED",
        }
    }
}
pub fn implementation_digest() -> xc_cache::ContentDigest {
    let sources = [
        include_str!("main.rs"),
        include_str!("journal.rs"),
        include_str!("source.rs"),
        include_str!("experiments.rs"),
        include_str!("transforms.rs"),
        include_str!("numerics.rs"),
        include_str!("../Cargo.toml"),
        include_str!("../Cargo.lock"),
    ];
    let normalized = sources
        .iter()
        .map(|s| s.replace("\r\n", "\n"))
        .collect::<Vec<_>>();
    xc_cache::ContentDigest::sha256(
        &serde_json::to_vec(&normalized).expect("source strings serialize"),
    )
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Check {
    pub name: String,
    pub category: String,
    pub status: Status,
    pub reason: String,
}
impl Check {
    pub fn new(name: &str, category: &str, status: Status, reason: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            category: category.into(),
            status,
            reason: reason.into(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Measurement {
    pub observable: String,
    pub checks: Vec<Check>,
    pub values: Value,
    #[serde(default)]
    pub source_manifests: Vec<xc_cache::ArtifactManifest>,
}
impl Measurement {
    pub fn new(observable: &str, values: Value) -> Self {
        Self {
            observable: observable.into(),
            checks: vec![],
            values,
            source_manifests: vec![],
        }
    }
    pub fn check(&mut self, name: &str, category: &str, status: Status, reason: impl Into<String>) {
        self.checks.push(Check::new(name, category, status, reason));
    }
    pub fn valid(&self) -> bool {
        !self.checks.iter().any(|c| {
            c.category == "validation" && matches!(c.status, Status::Fail | Status::Incomplete)
        })
    }
}
pub struct Journal {
    pub directory: PathBuf,
}
impl Journal {
    pub fn start(root: &Path, request: &Value) -> Result<Self> {
        fs::create_dir_all(root)?;
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let directory = root.join(format!("run-{time}-{}", std::process::id()));
        fs::create_dir(&directory)?;
        let this = Self { directory };
        this.save("request.json", request)?;
        this.save("build.json",&json!({"software_version":env!("CARGO_PKG_VERSION"),"implementation_digest":implementation_digest(),"toolkit_revision":crate::TOOLKIT_REVISION,"cargo_lock":include_str!("../Cargo.lock")}))?;
        println!("Run journal: {}", this.directory.display());
        Ok(this)
    }
    pub fn save(&self, name: &str, value: &impl Serialize) -> Result<()> {
        save(&self.directory.join(name), value)
    }
}
pub fn save(path: &Path, value: &impl Serialize) -> Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn negative_hypothesis_is_not_missing_data() {
        let mut m = Measurement::new("complex residual", json!({}));
        m.check(
            "complex-zero hypothesis",
            "hypothesis",
            Status::Fail,
            "nonzero imaginary residual",
        );
        assert!(m.valid());
        m.check(
            "quadrature refinement",
            "validation",
            Status::Incomplete,
            "unresolved",
        );
        assert!(!m.valid());
    }
    #[test]
    fn journals_never_overwrite_evidence() {
        let root = std::env::temp_dir().join(format!(
            "ccm-experiment-journal-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let j = Journal::start(&root, &json!({})).unwrap();
        assert!(j.save("request.json", &json!({"overwrite":true})).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}
