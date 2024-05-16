use crate::runner::{ExecCond, StackOptExec, Test, TestGroup};

pub fn optim_simpl() -> Test {
    let mut g = TestGroup::new("optim_simple");

    g.add(
        "drop test",
        StackOptExec::new(
            "$PCONST 123
        $PCONST 109
        $ADD
        $DROP
        ",
            vec![ExecCond::CheckStack(vec![])],
        ),
    );

    g.into()
}
