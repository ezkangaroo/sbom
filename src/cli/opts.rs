use clap::Parser;
use git_version::git_version;
use path_clean::PathClean;
use std::{
    env, io,
    path::{Path, PathBuf},
};
use tracing::Level;

const GIT_VERSION: &str = git_version!();

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Verbosity {
    Info,
    Debug,
    Trace,
}

impl Verbosity {
    pub fn to_level(&self) -> Level {
        match self {
            Verbosity::Info => Level::INFO,
            Verbosity::Debug => Level::DEBUG,
            Verbosity::Trace => Level::TRACE,
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, PartialEq, Eq)]
pub enum ScanOutputFormat {
    Text,
    Json,
}

#[derive(Parser, Debug)]
#[clap(author = "Humans", about = "Software Bill of Materials CLI", version=GIT_VERSION)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Command,

    #[arg(short, long, value_enum, help = "Set the verbosity level", default_value_t=Verbosity::Info, global=true)]
    pub verbosity: Verbosity,
}

#[derive(Parser, Debug)]
pub enum Command {
    Scan(Scan),
}

#[derive(Parser, Debug)]
/// Scan target to catalog and analyze dependencies. You can scan directory,
/// or file.
///
/// For example,
///     * ./sbom scan                   # scan cwd recursively
///     * ./sbom scan ./src             # scan dir recursively
///     * ./sbom scan ./src/main.py     # scan from single file
#[clap(verbatim_doc_comment)]
pub struct Scan {
    #[arg(value_parser = parse_target, default_value=".")]
    pub target: Option<Target>,

    #[arg(short, long, value_enum, help = "formatter to use", default_value_t=ScanOutputFormat::Text)]
    pub format: ScanOutputFormat,
}

#[derive(Debug, Clone)]
pub enum Target {
    Directory(PathBuf),
    File(PathBuf),
}

impl Target {
    pub fn path(&self) -> &PathBuf {
        match self {
            Self::Directory(p) => p,
            Self::File(p) => p,
        }
    }
}

fn parse_target(q: &str) -> Result<Target, std::io::Error> {
    let p = if q.is_empty() { "." } else { q.trim() };
    let path = absolute_path(PathBuf::from(p))?;

    if path.is_dir() {
        Ok(Target::Directory(path))
    } else if path.is_file() {
        Ok(Target::File(path))
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{}", path.display()),
        ))
    }
}

// https://stackoverflow.com/questions/30511331/getting-the-absolute-path-from-a-pathbuf
pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    };
    Ok(absolute_path.clean())
}
