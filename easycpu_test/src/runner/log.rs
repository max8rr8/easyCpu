use super::TestError;

#[derive(Debug, Clone)]
pub enum LogEntry {
    Passed,
    Failed(TestError),
}

#[derive(Debug, Clone)]
pub struct Logger {
    pub prefix: String,
}

const RED_COLOR: &str = "\u{001b}[31m";
const GREEN_COLOR: &str = "\u{001b}[32m";
const RESET_COLOR: &str = "\u{001b}[0m";

impl Logger {
    pub fn report_failure(&self, name: &str, err: TestError) {
        println!(
            "{RED_COLOR}[ FAILED ] {}::{}{RESET_COLOR}\n{}\n\n",
            self.prefix,
            name,
            err.to_string()
        );
        // let mut log = self.log.lock().unwrap();
        // log.push((self.prefix.clone(), LogEntry::Failed(err)));
    }

    pub fn report_success(&self, name: &str) {
        println!("{GREEN_COLOR}[ PASSED ] {}::{}{RESET_COLOR}", self.prefix, name);
    }

    pub fn create_nested(&self, name: impl AsRef<str>) -> Self {
        Logger {
            prefix: format!("{}::{}", self.prefix, name.as_ref()),
        }
    }
}
