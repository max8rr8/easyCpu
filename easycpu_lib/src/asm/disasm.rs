use crate::cpu;

pub fn disassemle_instruction(ins: cpu::Instruction) -> String {
    ins.to_string()
}
