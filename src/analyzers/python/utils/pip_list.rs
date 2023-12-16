use serde::{Deserialize, Serialize};

use crate::analyzers::analyze::{
    AnalysisUnit, Analyzable, AnalyzeKind, Analyzer, AnalyzerError, Dependency, DependencyKind,
};
use crate::analyzers::python::setuptools::SetupToolProject;
use crate::io::cmd::cmd_json;
use derive_more::Display;

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display(fmt = "PipListShowItem={}@{}", "name", "version")]
pub struct PipListShowItem {
    pub name: String,
    pub version: String,
}

pub struct PipListAnalyzer;

impl Analyzer<SetupToolProject> for PipListAnalyzer {
    fn kind(&self) -> AnalyzeKind {
        AnalyzeKind::Static
    }

    fn name(&self) -> &'static str {
        "piplist"
    }

    fn analyze(&self, project: &SetupToolProject) -> Result<AnalysisUnit, AnalyzerError> {
        let output: Vec<PipListShowItem> = cmd_json(
            "python3",
            vec!["-m", "pip", "list", "show", "--format", "json"],
            project.base_dir(),
            false,
        )?;

        let deps: Vec<_> = output
            .iter()
            .map(|p| Dependency {
                kind: DependencyKind::Pip,
                name: p.name.clone(),
                version: p.version.clone(),
                environment: Vec::default(),
            })
            .collect();

        let mut g = petgraph::graph::UnGraph::default();
        deps.iter().for_each(|d| {
            let _ = g.add_node(d.clone());
        });

        Ok(AnalysisUnit::builder()
            .analyzer((&self.name()).to_string())
            .project(project.to_target())
            .graph(g)
            .build())
    }
}
