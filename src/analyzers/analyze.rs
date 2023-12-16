use std::{fmt::Debug, path::PathBuf};

use petgraph::graph::UnGraph;
use serde::Serialize;
use thiserror::Error;
use typed_builder::TypedBuilder;

impl Serialize for AnalyzerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Debug)]
pub enum AnalyzeKind {
    Static,
    Dynamic,
}

#[derive(Clone, Serialize, TypedBuilder)]
pub struct AnalysisUnit {
    pub analyzer: String,
    pub project: ProjectTarget,
    pub graph: UnGraph<Dependency, ()>,
}

impl Debug for AnalysisUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnalysisUnit")
            .field("analyzer", &self.analyzer)
            .field("project", &self.project)
            .field("graph", &"<graph>")
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dependency {
    pub kind: DependencyKind,
    pub name: String,
    pub version: String,
    pub environment: Vec<Environment>,
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum DependencyKind {
    Pip,
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum Environment {
    Production,
    Development,
}

#[derive(Debug, Clone, Serialize)]
pub enum ProjectTargetKind {
    Setuptools,
}

#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct ProjectTarget {
    pub dir: PathBuf,
    pub manifest: PathBuf,
    pub kind: ProjectTargetKind,
}

pub trait Analyzer<P> {
    fn kind(&self) -> AnalyzeKind;
    fn name(&self) -> &'static str;
    fn analyze(&self, project: &P) -> Result<AnalysisUnit, AnalyzerError>;
}

pub trait Analyzable {
    fn analyze(&self) -> Result<AnalysisUnit, AnalysisError>;
    fn to_target(&self) -> ProjectTarget;
}

pub trait Scannable: Analyzable {}

#[derive(Error, Debug, Serialize)]
pub enum AnalysisError {
    #[error("Failed to analyze with all methods!")]
    AllMethodsFailed(Vec<AnalyzerError>),
}

#[derive(Error, Debug)]
pub enum AnalyzerError {
    #[error("Could not read file: {0}")]
    CouldNotRead(String),

    #[error("Could not execute command '{cmd}' in directory: {cwd}")]
    CouldNotExecute {
        cmd: String,
        cwd: String,
        cmd_std_err: String,
    },

    #[error("Could not analyze: {0}")]
    CouldNotAnalyzeErr(String),

    #[error("Is not implemented")]
    NotImplemented,

    #[error("Is not implemented")]
    CommandExecFailed(i32, String),

    #[error("Failed to execute command")]
    CommandFailed(#[from] std::io::Error),

    #[error("Failed to parse json")]
    FailedToParseFromJSON(#[from] serde_json::Error),
}
