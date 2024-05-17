use crate::runner::{test, CompilableTest, Test, TestGroup};

pub fn simple() -> Test {
    TestGroup::construct(
        "simple".to_owned(),
        vec![
            test!("empty", CompilableTest::new("")),
            test!("NOP", CompilableTest::new("NOP")),
            test!("ALU", CompilableTest::new("ADD ZX ZX ZX")),
            test!("consts", CompilableTest::new("0 0 0 0 123 \"213\"")),
        ],
    )
}

pub fn label() -> Test {
    TestGroup::construct(
        "label".to_owned(),
        vec![
            test!("just_label", CompilableTest::new("LABEL: NOP")),
            test!(
                "load_label",
                CompilableTest::new(
                    "BLABLBABL: NOP NOP
                    LLABEL R2 BLABLBABL",
                )
            ),
        ],
    )
}

pub fn stack() -> Test {
    TestGroup::construct(
        "label".to_owned(),
        vec![
            test!("init", CompilableTest::new("$INIT")),
            test!("basic", CompilableTest::new("$PUZX; $DUP; $DROP; $ADD")),
            test!("stackopt_empty", CompilableTest::new("@STACKOPT {}",)),
            test!(
                "stackopt_simple",
                CompilableTest::new(
                    "@STACKOPT {
                  $ADD
                }",
                )
            ),
        ],
    )
}
