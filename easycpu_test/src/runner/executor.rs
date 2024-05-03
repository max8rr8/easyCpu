use easycpu_lib::{
    cpu,
    exec::{ExecCpu, ExecStats},
};

use super::{log::PerformanceLog, CompilableTest, TestContext, TestError, Testable};

#[derive(Clone, Debug)]
pub enum ExecCond {
    CheckReg(cpu::Register, u16),
    SetReg(cpu::Register, u16),

    CheckMem(u16, u16),
    SetMem(u16, u16),

    CheckStack(Vec<u16>),
    SetStack(Vec<u16>),
}

impl ExecCond {
    pub fn check_cond(&self, cpu: &mut ExecCpu) -> Result<(), TestError> {
        match self {
            ExecCond::CheckReg(reg, val) => TestError::check_eq(
                format!("Register {}", reg.to_string()),
                *val,
                cpu.get_reg(*reg),
            ),

            ExecCond::CheckMem(addr, val) => {
                TestError::check_eq(format!("Memory {:#06x}", addr), *val, cpu.get_mem(*addr))
            }

            ExecCond::CheckStack(vals) => {
                let targ_sp = 0x4000u16 + (vals.len() as u16);
                TestError::check_eq(
                    String::from("SP position"),
                    targ_sp,
                    cpu.get_reg(cpu::Register::SP),
                )?;

                for (i, val) in vals.iter().enumerate() {
                    TestError::check_eq(
                        format!("stack element at {:#06x}", 0x4000u16 + (i as u16)),
                        *val,
                        cpu.get_mem(0x4000u16 + (i as u16)),
                    )?;
                }

                Ok(())
            }

            _ => Ok(()),
        }
    }

    pub fn apply_cond(&self, cpu: &mut ExecCpu) -> Result<(), TestError> {
        match self {
            ExecCond::SetReg(reg, val) => {
                cpu.set_reg(*reg, *val);
            }

            ExecCond::SetMem(addr, val) => {
                cpu.set_mem(*addr, *val);
            }

            ExecCond::SetStack(vals) => {
                cpu.set_reg(cpu::Register::SP, 0x4000u16 + (vals.len() as u16));
                cpu.set_reg(cpu::Register::LP, 0x4000u16);
                for (i, val) in vals.iter().enumerate() {
                    cpu.set_mem(0x4000u16 + (i as u16), *val);
                }
            }

            _ => {}
        }

        Ok(())
    }
}

pub struct Executor {
    code: String,
    combos: Vec<Vec<ExecCond>>,
}

impl Executor {
    pub fn new(code: impl Into<String>, conds: Vec<ExecCond>) -> Executor {
        Executor {
            code: code.into() + " \nHALT",
            combos: vec![conds],
        }
    }

    pub fn validate_conds(mut cpu: ExecCpu, conds: &Vec<ExecCond>) -> Result<(), TestError> {
        for cond in conds {
            cond.check_cond(&mut cpu)?;
        }

        Ok(())
    }

    pub fn add_case(mut self, conds: Vec<ExecCond>) -> Self {
        self.combos.push(conds);
        self
    }
}

impl Testable for Executor {
    fn run(&self, ctx: &TestContext) -> Result<(), TestError> {
        let compiled = CompilableTest::compile(&self.code)?;
        let program_len = compiled.len();

        let cpu = ExecCpu::new(compiled);

        let mut stats = ExecStats {
            ..Default::default()
        };

        for cons in &self.combos {
            let mut cpu = cpu.clone();

            for c in cons {
                c.apply_cond(&mut cpu)?;
            }

            let mut lim = 0xf000;
            while cpu.get_mem(0xffff) != 0 {
                cpu.exec_next();

                lim -= 1;
                if lim == 0 {
                    return Err(TestError::TimedOut);
                }
            }

            stats += cpu.get_stats();
            Self::validate_conds(cpu, cons)?;
        }

        ctx.log.report_perf(PerformanceLog {
            exec: stats,
            program_len,
        });

        Ok(())
    }
}
