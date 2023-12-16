use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::debug;

use super::opts::{Scan, ScanOutputFormat, Target};
use crate::analyzers::{
    analyze::{AnalysisError, AnalysisUnit},
    python::setuptools::*,
};

pub fn run_scan(args: Scan) {
    let base_dir = args.target.expect("some target");
    let findings = scan(base_dir);

    for finding in findings {
        debug!("{:?}", finding);
        if args.format == ScanOutputFormat::Json {
            let out = serde_json::to_string(&finding).expect("to serialize");
            println!("{}", out);
        }
    }
}

pub fn scan(target: Target) -> Vec<Result<AnalysisUnit, AnalysisError>> {
    vec![discover_setuptools]
        .par_iter()
        .map(|d| {
            let projects = d(target.path());
            let mut project_findings = Vec::default();
            for project in projects {
                project_findings.push(project.analyze())
            }

            project_findings
        })
        .flatten_iter()
        .collect()
}
