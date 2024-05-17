use super::{Test, TestContext, TestError, Testable};

pub struct TestGroup {
    name: String,
    tests: Vec<Test>,
}

impl TestGroup {
    pub fn new(name: impl Into<String>) -> TestGroup {
        TestGroup {
            name: name.into(),
            tests: vec![],
        }
    }

    pub fn construct(name: impl Into<String>, tests: Vec<Test>) -> Test {
        let name: String = name.into();
        Test::new(name.clone(), TestGroup { name, tests })
    }

    pub fn add_from_testable(&mut self, name: impl Into<String>, test: impl Testable + 'static) {
        self.tests.push(Test::new(name.into(), test));
    }

    pub fn add(&mut self, test: Test) {
        self.tests.push(test);
    }
}

impl From<TestGroup> for Test {
    fn from(val: TestGroup) -> Self {
        Test::new(val.name.clone(), val)
    }
}

impl Testable for TestGroup {
    fn run(&self, ctx: &TestContext) -> Result<(), super::TestError> {
        let log = ctx.log.create_nested("::");

        let mut had_error = false;
        for test in &self.tests {
            had_error |= test.run(&log).is_err();
        }

        if had_error {
            Err(TestError::Elevating)
        } else {
            Ok(())
        }
    }
}
