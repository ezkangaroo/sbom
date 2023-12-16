use super::opts::{Scan, ScanOutputFormat, Target};
use crate::analyzers::{
    python::setuptools::*,
    spec::{Unit, Units},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn run_scan(args: Scan) {
    let base_dir = args.target.expect("some target");
    let findings = scan(base_dir);

    match args.format {
        ScanOutputFormat::Json => findings.json(),
        ScanOutputFormat::Text => findings.summrize(),
    }
}

pub fn scan(target: Target) -> Units {
    let res = vec![discover_setuptools]
        .par_iter()
        .map(|d| {
            let projects = d(target.path());
            let mut project_findings = Vec::default();
            for project in projects {
                let project_target = project.to_target();
                let findings = project.analyze();
                project_findings.push(Unit {
                    target: project_target,
                    findings: Some(findings),
                })
            }

            project_findings
        })
        .flatten_iter()
        .collect();

    Units(res)
}
