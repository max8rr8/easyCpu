use crate::runner::{Test, TestGroup};

mod simple;

pub fn stack_test() -> Test {
    TestGroup::construct(
        "stack".to_owned(),
        vec![simple::simple()],
    )
}
