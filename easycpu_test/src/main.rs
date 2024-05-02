use crate::runner::{TestContext, TestGroup};

pub mod runner;
pub mod compilation;
pub mod exec;

fn main() {
    let tests = TestGroup::construct("", vec![
        compilation::compilation_test(),
        exec::exec_test(),
    ]);

    tests.run(&TestContext {
        log: runner::Logger { prefix: String::from("") }
    });
}
