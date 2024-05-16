use crate::runner::{Test, TestGroup};

mod simple;
mod optim;

pub fn stack_test() -> Test {
    TestGroup::construct(
        "stack".to_owned(),
        vec![simple::simple(), simple::funcs(), optim::optim_simpl()],
    )
}
