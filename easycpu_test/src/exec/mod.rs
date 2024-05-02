use crate::runner::{Test, TestGroup};

mod simple;

pub fn exec_test() -> Test {
    TestGroup::construct(
        "exec".to_owned(),
        vec![simple::simple(), simple::stack()],
    )
}
