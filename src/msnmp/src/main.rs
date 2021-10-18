//! Main doc.
use anyhow::Result;
use msnmp::{self, Params};
use structopt::StructOpt;

fn main() -> Result<()> {
    let args = Params::from_args();
    msnmp::run(args)?;

    Ok(())
}
