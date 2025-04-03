use clap::Args;
use crate::{config::OxideConfig, utils::file_ops};

#[derive(Args)]
pub struct DeployArgs {
    #[arg(short, long)]
    destination: String,
}

pub fn execute(args: DeployArgs) -> anyhow::Result<()> {
    let config = OxideConfig::load()?;
    let dest_path = std::path::Path::new(&args.destination);
    
    file_ops::ensure_dir(dest_path)?;
    file_ops::recursive_copy(&config.build.out_dir, dest_path)?;
    
    Ok(())
}
