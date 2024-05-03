use std::{
    fs,
    io::{self, Write},
};

use runner::{Logger, PerformanceLog};

use crate::runner::TestGroup;

pub mod runner;

pub mod compilation;
pub mod exec;
pub mod stack;

fn write_perf(path: String, perf: &[(String, PerformanceLog)]) -> Result<(), io::Error> {
    let mut f = fs::File::create(path)?;

    f.write_all(b"test_name,proglen,cycles,nop,alu,load,store,branch\n")?;
    for (name, entry) in perf.iter() {
        let cycles = entry.exec.nop
            + entry.exec.alu
            + entry.exec.load
            + entry.exec.store
            + entry.exec.branch;
        writeln!(
            f,
            "\"{}\",{},{},{},{},{},{},{}",
            name,
            entry.program_len,
            cycles,
            entry.exec.nop,
            entry.exec.alu,
            entry.exec.load,
            entry.exec.store,
            entry.exec.branch
        )?;
    }

    Ok(())
}

fn main() {
    let tests = TestGroup::construct(
        "",
        vec![
            compilation::compilation_test(),
            exec::exec_test(),
            stack::stack_test(),
        ],
    );

    let log = Logger::new();
    let _ = tests.run(&log);

    if let Ok(path) = std::env::var("PERF_PATH") {
        let perf = log.performance.lock().unwrap();
        if let Err(e) = write_perf(path, &perf) {
            eprintln!("Failed to write performance data: {}", e);
        }
    };
}
