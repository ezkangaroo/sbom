use super::analyze::{AnalysisError, AnalysisUnit, ProjectTarget};
use clap::crate_version;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Units(pub Vec<Unit>);

#[derive(Debug, Serialize)]
pub struct Unit {
    pub findings: Option<Result<AnalysisUnit, AnalysisError>>,
    pub target: ProjectTarget,
}

impl Unit {
    pub fn status(&self) -> String {
        match self.findings {
            None => "skipped",
            Some(Ok(_)) => "analyzed",
            Some(Err(_)) => "failed",
        }
        .to_string()
    }
    pub fn summarize(&self) -> String {
        format!(
            "{:?} ({:?}) [{}]",
            self.target.dir,
            self.target.kind,
            self.status()
        )
    }
}

impl Units {
    pub fn summrize(&self) {
        let version = crate_version!();
        println!("\n");
        println!("sbom cli version: {version}\n");

        for unit in self.0.iter() {
            println!("- {}", unit.summarize())
        }

        println!("\n");
    }

    pub fn json(&self) {
        println!("{}", serde_json::to_string(self).expect("to serialize"))
    }
}
