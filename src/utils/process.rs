use std::process::{Command, Output};
use crate::error::OxideError;

pub struct CommandExt {
    inner: Command,
}

impl CommandExt {
    pub fn new(program: &str) -> Self {
        Self {
            inner: Command::new(program),
        }
    }

    pub fn arg(&mut self, arg: &str) -> &mut Self {
        self.inner.arg(arg);
        self
    }

    pub fn exec(&mut self) -> anyhow::Result<Output> {
        let output = self.inner.output()?;
        if !output.status.success() {
            return Err(OoxideError::CommandError.into());
        }
        Ok(output)
    }
}
