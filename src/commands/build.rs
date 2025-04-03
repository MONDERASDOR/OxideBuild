use crate::{config::config_parser::OxideConfig, platform::target::TargetPlatform};
use clap::Args;

#[derive(Args)]
pub struct BuildArgs {
    #[arg(short, long)]
    profile: Option<String>,
    #[arg(short, long)]
    target: Option<String>,
}

pub fn execute(args: BuildArgs) -> anyhow::Result<()> {
    let config = OxideConfig::load()?;
    let target_platform = TargetPlatform::detect_or_select(args.target)?;
    
    target_platform.build(&config, args.profile.as_deref())?;
    
    Ok(())
}
