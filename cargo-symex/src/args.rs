use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Reads from elf file
    #[clap(
        long,
        conflicts_with = "bin",
        conflicts_with = "example",
        conflicts_with = "lib",
        conflicts_with = "release",
        conflicts_with = "features",
        conflicts_with = "all_features",
        conflicts_with = "subcommand",
        conflicts_with = "embed_bitcode"
    )]
    pub path: Option<String>,

    #[clap(long)]
    pub elf: bool,

    /// Build package library.
    #[clap(long, conflicts_with = "bin", conflicts_with = "example")]
    pub lib: Option<bool>,

    /// Builds given example.
    #[clap(long, conflicts_with = "bin", conflicts_with = "lib")]
    pub example: Option<String>,

    /// Builds given binary.
    #[clap(long, conflicts_with = "example", conflicts_with = "lib")]
    pub bin: Option<String>,

    /// Build in release mode.
    #[clap(long)]
    pub release: bool,

    /// List of features to activate.
    #[clap(long)]
    pub features: Vec<String>,

    /// Activate all features.
    #[clap(long)]
    pub all_features: bool,

    /// Name of function to run. Should be a full module path, excluding the root module.
    #[clap(short, long)]
    pub function: Option<String>,

    #[clap(subcommand)]
    pub subcommand: Option<Subcommands>,

    #[clap(short, long)]
    pub embed_bitcode: Option<bool>,
}

#[derive(Parser, Debug)]
pub enum Subcommands {
    /// Compile with Clang.
    C(ClangArgs),
}

#[derive(Parser, Debug)]
pub struct ClangArgs {
    /// Path to c-file to build.
    #[clap(name = "File")]
    pub path: PathBuf,

    /// Name of function to run.
    #[clap(long, short)]
    pub function: Option<String>,
}
