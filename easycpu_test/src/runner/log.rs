use std::sync::{Arc, Mutex};

use easycpu_lib::exec::ExecStats;

use super::TestError;

#[derive(Debug, Clone)]
pub enum LogEntry {
    Passed,
    Failed(TestError),
}

#[derive(Debug, Clone)]
pub struct PerformanceLog {
    pub exec: ExecStats,
    pub program_len: usize,
}

#[derive(Debug, Clone)]
pub struct Logger {
    pub name: String,
    pub position: String,
    pub performance: Arc<Mutex<Vec<(String, PerformanceLog)>>>,
}

const RED_COLOR: &str = "\u{001b}[31m";
const GREEN_COLOR: &str = "\u{001b}[32m";
const RESET_COLOR: &str = "\u{001b}[0m";

impl Logger {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            name: String::from(""),
            position: String::from(""),
            performance: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn report_failure(&self, err: TestError) {
        if let TestError::Elevating = err {
            println!("{RED_COLOR}[ FAILED ] {}{RESET_COLOR}", self.name,);
        } else {
            println!("{RED_COLOR}[ FAILED ] {}{RESET_COLOR}", self.name);

            if !self.position.is_empty() {
                println!("Test at {}", self.position);
            }

            println!("{}\n", err.to_string());
        }
    }

    pub fn report_success(&self) {
        println!("{GREEN_COLOR}[ PASSED ] {}{RESET_COLOR}", self.name);
    }

    pub fn report_perf(&self, stats: PerformanceLog) {
        let mut perf = self.performance.lock().unwrap();
        perf.push((self.name.clone(), stats));
    }

    pub fn create_nested(&self, name: impl AsRef<str>) -> Self {
        let name = name.as_ref();

        Logger {
            name: self.name.clone() + name,
            ..self.clone()
        }
    }

    pub fn with_position(mut self, position: impl AsRef<str>) -> Self {
        self.position = position.as_ref().to_string();
        self
    }
}
