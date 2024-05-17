use crate::runner::{test, ExecCond, StackOptExec, Test, TestGroup};

pub fn optim_simpl() -> Test {
    let mut g = TestGroup::new("optim_simple");

    g.add(test!(
        "drop_test",
        StackOptExec::new(
            "$PCONST 123
        $PCONST 109
        $ADD
        $DROP
        ",
            vec![ExecCond::CheckStack(vec![])],
        )
    ));

    g.add(test!(
        "plabel_shift",
        StackOptExec::new(
            "$PLABEL DATA
      $PCONST 2
      $ADD
      $ACONST 1
      $LOAD
      }
      DATA: 1 2 3 4 5 6  
      {
      ",
            vec![ExecCond::CheckStack(vec![4])],
        )
    ));

    g.into()
}
