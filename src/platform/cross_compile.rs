use crate::{config::OxideConfig, utils::process, error::OxideError};

pub fn setup_cross_compilation(target: &str) -> anyhow::Result<()> {
    let mut cmd = process::Command::new("rustup");
    cmd.arg("target")
       .arg("add")
       .arg(target);
    
    cmd.exec().map_err(|_| OxideError::CrossCompileSetupError)?;
    Ok(())
}
