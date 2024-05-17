use super::{Logger, TestError};

#[derive(Debug, Clone)]
pub struct TestContext {
    pub log: Logger,
}

pub trait Testable {
    fn run(&self, ctx: &TestContext) -> Result<(), TestError>;
}

pub struct Test {
    name: String,
    testable: Box<dyn Testable>,
    position: String,
}

impl Test {
    pub fn new(name: impl ToString, testable: impl Testable + 'static) -> Self {
        Self {
            name: name.to_string(),
            testable: Box::new(testable),
            position: String::new(),
        }
    }

    pub fn run(&self, parent_log: &Logger) -> Result<(), TestError> {
        let ctx = TestContext {
            log: parent_log
                .create_nested(&self.name)
                .with_position(&self.position),
        };

        match self.testable.run(&ctx) {
            Ok(()) => {
                ctx.log.report_success();
                Ok(())
            }
            Err(err) => {
                ctx.log.report_failure(err.clone());
                Err(err)
            }
        }
    }

    pub fn with_position(mut self, position: impl ToString) -> Self {
        self.position = position.to_string();
        self
    }
}

#[macro_export]
macro_rules! test {
    ($name: expr, $testable: expr) => {{
        let pos = format!("{}:{}:{}", file!(), line!(), column!());
        Test::new($name.to_string(), $testable).with_position(pos)
    }};
}

pub use test;
