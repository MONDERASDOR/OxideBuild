use clap::Args;
use crate::utils::process;

#[derive(Args)]
pub struct TestArgs {
    #[arg(short, long)]
    nocapture: bool,
}

pub fn execute(args: TestArgs) -> anyhow::Result<()> {
    let mut cmd = process::Command::new("cargo");
    cmd.arg("test");
    
    if args.nocapture {
        cmd.arg("--nocapture");
    }
    
    cmd.exec()?;
    Ok(())
}
