use crate::runner::{CompilableTest, Test, TestGroup};

pub fn simple() -> Test {
    TestGroup::construct(
        "simple".to_owned(),
        vec![
            Test::new("empty", CompilableTest::new("")),
            Test::new("NOP", CompilableTest::new("NOP")),
            Test::new("ALU", CompilableTest::new("ADD ZX ZX ZX")),
            Test::new("consts", CompilableTest::new("0 0 0 0 123 \"213\"")),
        ],
    )
}

pub fn label() -> Test {
    TestGroup::construct(
        "label".to_owned(),
        vec![
            Test::new("just_label", CompilableTest::new("LABEL: NOP")),
            Test::new(
                "load_label",
                CompilableTest::new(
                    "BLABLBABL: NOP NOP
                    LLABEL R2 BLABLBABL",
                ),
            ),
        ],
    )
}

pub fn stack() -> Test {
  TestGroup::construct(
      "label".to_owned(),
      vec![
          Test::new("init", CompilableTest::new("$INIT")),
          Test::new("basic", CompilableTest::new("$PUZX; $DUP; $DROP; $ADD")),
          Test::new(
              "stackopt_empty",
              CompilableTest::new(
                  "@STACKOPT {}",
              ),
          ),
          Test::new(
            "stackopt_simple",
            CompilableTest::new(
                "@STACKOPT {
                  $ADD
                }",
            ),
        ),
      ],
  )
}

