//! Main doc.
use anyhow::Result;
use msnmp::{self, Params};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Params::from_args();
    msnmp::run(args).await?;

    Ok(())
}
