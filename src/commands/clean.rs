use crate::utils::file_ops;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct CleanArgs {
    #[arg(short, long)]
    all: bool,
}

pub fn execute(args: CleanArgs) -> anyhow::Result<()> {
    let config = crate::config::OxideConfig::load()?;
    
    if args.all {
        file_ops::remove_dir_all(&config.build.out_dir)?;
    } else {
        let profile_dirs = file_ops::find_files(&format!("{}/*", config.build.out_dir.display()))?;
        for dir in profile_dirs {
            if dir.is_dir() {
                file_ops::remove_dir_all(&dir)?;
            }
        }
    }
    
    Ok(())
}
