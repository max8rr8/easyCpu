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
}

impl Test {
    pub fn new(name: impl ToString, testable: impl Testable + 'static) -> Self {
        Self {
            name: name.to_string(),
            testable: Box::new(testable),
        }
    }

    pub fn run(&self, parent_log: &Logger) -> Result<(), TestError> {
        let ctx = TestContext {
            log: parent_log.create_nested(&self.name),
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
}
