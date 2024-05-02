use super::{TestContext, TestError, Testable};

pub struct CompilableTest {
    code: String,
}

impl CompilableTest {
    pub fn new(code: impl Into<String>) -> CompilableTest {
        CompilableTest { code: code.into() }
    }

    pub fn compile(code: &str) -> Result<Vec<u16>, TestError> {
        let errors = easycpu_lib::asm::parse_and_compile(code);
        match errors {
            Err(e) => Err(TestError::CompilationError(
                e.into_iter()
                    .map(|e| format!("Error at {}: {:#?}", e.start_pos, e.error))
                    .collect::<Vec<String>>()
                    .join("\n"),
            )),
            Ok(res) => Ok(res),
        }
    }
}

impl Testable for CompilableTest {
    fn run(&self, _: &TestContext) -> Result<(), TestError> {
        Self::compile(&self.code).map(|_| ())
    }
}
