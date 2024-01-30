use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
    str,
};

use crate::test_suite::TestCase;

pub struct EngineWrapper {
    path: PathBuf,
}

impl EngineWrapper {
    pub fn new(path: PathBuf) -> Self {
        if !Path::new(&path).exists() {
            panic!("Could not find file {:?}", path);
        }
        Self { path }
    }

    pub fn run(&self, case: &TestCase) -> Result<Option<usize>, Box<dyn Error>> {
        let mut cmd: Command = Command::new(&self.path);
        cmd.args(&[&case.depth.to_string(), &case.fen]);
        let _out = cmd.output()?;
        //println!("{}", String::from_utf8(_out.stdout).unwrap());
        Ok(None)
    }

    pub fn name(&self) -> &str {
        self.path.file_stem().unwrap().to_str().unwrap()
    }
}
