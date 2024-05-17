use easycpu_lib::cpu::Register;

use crate::runner::{test, ExecCond, Executor, Test, TestGroup};

pub fn simple() -> Test {
    let mut g = TestGroup::new("simple");

    g.add(test!("empty", Executor::new("", vec![])));

    g.add(test!(
        "pconst_1234",
        Executor::new(
            "LCONST r2 0x1234",
            vec![ExecCond::CheckReg(Register::R2, 0x1234)],
        )
    ));

    g.add(test!(
        "add",
        Executor::new(
            "ADD r2 r3 r2",
            vec![
                ExecCond::SetReg(Register::R2, 0x10),
                ExecCond::SetReg(Register::R3, 0x30),
                ExecCond::CheckReg(Register::R2, 0x40),
            ],
        )
        .add_case(vec![
            ExecCond::SetReg(Register::R2, 0x800),
            ExecCond::SetReg(Register::R3, 0x30),
            ExecCond::CheckReg(Register::R2, 0x830),
        ])
    ));

    g.add(test!(
        "store",
        Executor::new(
            "LCONST r2 0x1234; STORE r2 r3 +2",
            vec![
                ExecCond::SetReg(Register::R3, 0x4000),
                ExecCond::CheckMem(0x4002, 0x1234),
            ],
        )
    ));

    g.into()
}

pub fn stack() -> Test {
    let mut g = TestGroup::new("stack");

    g.add(test!(
        "init",
        Executor::new("$INIT", vec![ExecCond::CheckStack(vec![])])
    ));

    g.add(test!(
        "push_reg",
        Executor::new(
            "$INIT; $PUSH R2",
            vec![
                ExecCond::SetReg(Register::R2, 0x1234),
                ExecCond::CheckStack(vec![0x1234]),
            ],
        )
    ));

    g.add(test!(
        "pop_reg",
        Executor::new(
            "$POP R2",
            vec![
                ExecCond::SetStack(vec![0x1234]),
                ExecCond::CheckReg(Register::R2, 0x1234),
                ExecCond::CheckStack(vec![]),
            ],
        )
    ));

    g.into()
}
