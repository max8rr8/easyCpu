use crate::cpu::{self, AluInstruction, MemInstruction, BranchInstruction};

fn generate_flags(names: &str, flags: [bool; 3]) -> String {
    let flags = names
        .char_indices()
        .filter(|(i, c)| flags[*i])
        .map(|(i, x)| x);

    let flags = ".".chars().chain(flags);
    let flags: String = flags.collect();

    if flags.len() == 1 {
        return String::from("");
    } else {
        return flags;
    }
}

trait Dissassemble {
    fn dissassemble(&self) -> String;
}

impl Dissassemble for AluInstruction {
    fn dissassemble(&self) -> String {
        format!(
            "{} {} {} {}",
            generate_flags("xyo", [self.nx, self.ny, self.no]),
            self.dst.to_string(),
            self.src_a.to_string(),
            self.src_b.to_string(),
        )
    }
}

impl Dissassemble for MemInstruction {
    fn dissassemble(&self) -> String {
        format!(
            "{} {} {} {}",
            generate_flags("hls", [self.hi, self.lo, self.sw]),
            self.dst.to_string(),
            self.addr.to_string(),
            self.shift,
        )
    }
}

impl Dissassemble for BranchInstruction {
  fn dissassemble(&self) -> String {
      format!(
          "{} {} {}",
          generate_flags("egl", [self.eq, self.gt, self.lt]),
          self.cond.to_string(),
          self.shift,
      )
  }
}


pub fn disassemle_instruction(ins: cpu::Instruction) -> String {
    match ins {
        cpu::Instruction::NOP => String::from("NOP"),
        cpu::Instruction::ADD(o) => format!("ADD{}", o.dissassemble()),
        cpu::Instruction::AND(o) => format!("AND{}", o.dissassemble()),
        cpu::Instruction::LOAD(o) => format!("LOAD{}", o.dissassemble()),
        cpu::Instruction::STORE(o) => format!("STORE{}", o.dissassemble()),
        cpu::Instruction::BRANCH(o) => format!("BRANCH{}", o.dissassemble()),
        cpu::Instruction::CUSTOM(o) => format!("0x{:x}", o),
    }
}
