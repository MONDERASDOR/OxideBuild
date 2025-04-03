use crate::{config::OxideConfig, utils::process};

pub struct LinuxPlatform;

impl super::target::TargetPlatform for LinuxPlatform {
    fn detect() -> anyhow::Result<super::Platform> {
        Ok(super::Platform::Linux)
    }

    fn build(&self, config: &OxideConfig, profile: Option<&str>) -> anyhow::Result<()> {
        let profile = profile.unwrap_or("debug");
        let mut cmd = process::Command::new("cargo");
        
        cmd.arg("build")
           .arg("--target=x86_64-unknown-linux-gnu");
        
        if profile == "release" {
            cmd.arg("--release");
        }
        
        cmd.exec()?;
        
        let out_dir = config.build.out_dir.join(profile);
        std::fs::create_dir_all(&out_dir)?;
        
        let target_path = format!("target/x86_64-unknown-linux-gnu/{}/{}", profile, config.project.name);
        std::fs::copy(target_path, out_dir.join(&config.project.name))?;
        
        Ok(())
    }
}
