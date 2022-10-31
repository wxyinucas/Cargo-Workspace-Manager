use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[clap(author = "Rex Wang", version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action,

    #[clap(short, long)]
    pub debug: bool,

    #[clap(long)]
    pub loc: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    Add(Add),
    Fix(Fix),
    Rename(Rename),
    Delete(Delete),
    New(New),
    // Init(init), todo: new function
}

#[derive(Debug, Parser)]
pub struct Add {
    pub(crate) name: String,
}

#[derive(Debug, Parser)]
pub struct Fix;

#[derive(Debug, Parser)]
pub struct Rename {
    pub(crate) old: String,
    pub(crate) new: String,
}

#[derive(Debug, Parser)]
pub struct Delete {
    pub(crate) name: String,
}

#[derive(Debug, Parser)]
pub struct New;