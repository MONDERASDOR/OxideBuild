use crate::{config::OxideConfig, utils::process};

pub struct WindowsPlatform;

impl super::target::TargetPlatform for WindowsPlatform {
    fn detect() -> anyhow::Result<super::Platform> {
        Ok(super::Platform::Windows)
    }

    fn build(&self, config: &OxideConfig, profile: Option<&str>) -> anyhow::Result<()> {
        let profile = profile.unwrap_or("debug");
        let mut cmd = process::Command::new("cargo");
        
        cmd.arg("build")
           .arg("--target=x86_64-pc-windows-msvc");
        
        if profile == "release" {
            cmd.arg("--release");
        }
        
        cmd.exec()?;
        
        let out_dir = config.build.out_dir.join(profile);
        std::fs::create_dir_all(&out_dir)?;
        
        let target_path = format!("target/x86_64-pc-windows-msvc/{}/{}.exe", profile, config.project.name);
        std::fs::copy(target_path, out_dir.join(format!("{}.exe", config.project.name)))?;
        
        Ok(())
    }
}
