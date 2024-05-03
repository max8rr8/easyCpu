#[derive(Debug, Clone)]
pub enum TestError {
    CompilationError(String),
    InvalidResult(String),
    TimedOut,
    Elevating,
}

impl ToString for TestError {
    fn to_string(&self) -> String {
        match self {
            TestError::CompilationError(e) => format!("Failed to compile: {}", e),
            TestError::InvalidResult(e) => format!("Invalid {}", e),
            TestError::TimedOut => "Timed out".to_owned(),
            TestError::Elevating => String::from(""),
        }
    }
}

impl TestError {
    pub fn check_eq(name: String, expected: u16, actual: u16) -> Result<(), TestError> {
        if expected != actual {
            Err(TestError::InvalidResult(format!(
                "{}: {:#06x} != {:#06x}",
                name, expected, actual
            )))
        } else {
            Ok(())
        }
    }
}
