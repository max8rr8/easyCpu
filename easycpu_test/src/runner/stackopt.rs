use super::{ExecCond, Executor, TestContext, TestError, Testable};

pub struct StackOptExec {
    nonopt: Executor,
    yesopt: Executor,
}

impl StackOptExec {
    fn wrap_conds(conds: &[ExecCond]) -> Vec<ExecCond> {
        let mut new = Vec::new();
        new.push(ExecCond::SetStack(vec![]));
        new.extend(conds.iter().cloned());
        new
    }

    pub fn new(code: impl Into<String>, conds: Vec<ExecCond>) -> StackOptExec {
        let code = code.into();
        let opton = format!("@STACKOPT {{\n{}\n}}\n", code);
        StackOptExec {
            nonopt: Executor::new(format!("{{ {} }}", code), Self::wrap_conds(&conds)),
            yesopt: Executor::new(opton, Self::wrap_conds(&conds)),
        }
    }

    pub fn add_case(self, conds: Vec<ExecCond>) -> Self {
        StackOptExec {
            nonopt: self.nonopt.add_case(Self::wrap_conds(&conds)),
            yesopt: self.yesopt.add_case(Self::wrap_conds(&conds)),
        }
    }
}

impl Testable for StackOptExec {
    fn run(&self, ctx: &TestContext) -> Result<(), TestError> {
        self.nonopt.run(ctx)?;

        let nested = ctx.log.create_nested("$STACKOPT");
        let optctx = TestContext { log: nested };

        if let Err(err) = self.yesopt.run(&optctx) {
            optctx.log.report_failure(err.clone());
            return Err(TestError::Elevating);
        };

        Ok(())
    }
}
