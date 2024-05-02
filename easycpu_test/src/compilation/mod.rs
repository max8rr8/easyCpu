use crate::runner::{Test, TestGroup};

mod simple;

pub fn compilation_test() -> Test {
    TestGroup::construct(
        "compilation".to_owned(),
        vec![simple::simple(), simple::label(), simple::stack()],
    )
}
