use crate::analyzers::analyze::{
    AnalysisError, AnalysisUnit, Analyzable, Analyzer, ProjectTarget, ProjectTargetKind, Scannable,
};
use crate::analyzers::python::utils::pip_list::PipListAnalyzer;
use derive_more::Display;
use glob::glob;
use itertools::Itertools;
use std::path::{Path, PathBuf};
use tracing::debug;
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, TypedBuilder, Display)]
#[display(fmt = "SetupToolProject@{}", "manifest.display()")]
pub struct SetupToolProject {
    pub manifest: PathBuf,
}

impl SetupToolProject {
    pub fn base_dir(&self) -> PathBuf {
        self.manifest
            .parent()
            .expect("parent directory")
            .to_path_buf()
    }

    pub fn to_project_target(&self) -> ProjectTarget {
        ProjectTarget::builder()
            .dir(self.base_dir())
            .manifest(self.manifest.clone())
            .kind(ProjectTargetKind::Setuptools)
            .build()
    }
}

pub fn discover_setuptools(target: &Path) -> Vec<Box<dyn Scannable>> {
    let pattern = format!("{}/**/*.txt", target.display());

    let (found, _failure): (Vec<_>, Vec<_>) = glob(pattern.as_str())
        .expect("to compile glob")
        .partition_result();

    let mut projects: Vec<Box<dyn Scannable>> = Vec::default();
    for path in found {
        let project = SetupToolProject::builder().manifest(path.clone()).build();
        debug!("found project={project}");

        projects.push(Box::new(project))
    }

    projects
}

impl Scannable for SetupToolProject {}

impl Analyzable for SetupToolProject {
    fn analyze(&self) -> Result<AnalysisUnit, AnalysisError> {
        let mut errs = Vec::default();
        let analyzers = vec![PipListAnalyzer];
        for analyzer in analyzers {
            let res = analyzer.analyze(self);
            match res {
                Ok(r) => return Ok(r),
                Err(e) => errs.push(e),
            }
        }

        Err(AnalysisError::AllMethodsFailed(errs))
    }
}
