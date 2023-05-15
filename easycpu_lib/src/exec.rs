use std::fmt::format;

use crate::cpu::{self, CpuState};

enum ExecEvent {
    NONE,
    JUMP(u16),
    MEMGET(u16, u16),
    MEMSET(u16, u16),
}

impl ToString for ExecEvent {
    fn to_string(&self) -> String {
        match self {
            ExecEvent::NONE => String::from(""),
            ExecEvent::JUMP(addr) => format!("JMP => {:04x}", addr),
            ExecEvent::MEMGET(addr, val) => format!("MEMGET({:04x}) => {:04x}", addr, val),
            ExecEvent::MEMSET(addr, val) => format!("MEMSET({:04x}) <= {:04x}", addr, val),
        }
    }
}

pub struct ExecCpu {
    pc: u16,
    registers: [u16; 6],
    mem: Vec<u16>,
    last_event: ExecEvent,
    jumped: bool,
}


impl CpuState for ExecCpu {
    fn get_reg(&mut self, reg: crate::cpu::Register) -> u16 {
        match reg {
            cpu::Register::ZX => 0,
            cpu::Register::PC => self.pc,
            cpu::Register::R2 => self.registers[0],
            cpu::Register::R3 => self.registers[1],
            cpu::Register::R4 => self.registers[2],
            cpu::Register::R5 => self.registers[3],
            cpu::Register::LP => self.registers[4],
            cpu::Register::SP => self.registers[5],
        }
    }

    fn set_reg(&mut self, reg: crate::cpu::Register, val: u16) {

        match reg {
            cpu::Register::ZX => (),
            cpu::Register::PC => {
                self.jumped = true;
                self.last_event = ExecEvent::JUMP(val);
                self.pc = val;
            }
            cpu::Register::R2 => self.registers[0] = val,
            cpu::Register::R3 => self.registers[1] = val,
            cpu::Register::R4 => self.registers[2] = val,
            cpu::Register::R5 => self.registers[3] = val,
            cpu::Register::LP => self.registers[4] = val,
            cpu::Register::SP => self.registers[5] = val,
        }
    }

    fn get_mem(&mut self, addr: u16) -> u16 {
        let val = *self.mem.get(addr as usize).unwrap_or(&0);
        self.last_event = ExecEvent::MEMGET(addr, val);

        val
    }

    fn set_mem(&mut self, addr: u16, val: u16) {
        self.last_event = ExecEvent::MEMSET(addr, val);
        self.mem[addr as usize] = val
    }
}

impl ExecCpu {
    pub fn new(mut init_ram: Vec<u16>) -> Self {
        init_ram.resize(0xffff + 1, 0);
        init_ram[0xffff] = 0xffff;
        Self {
            pc: 0,
            registers: [0; 6],
            mem: init_ram,
            last_event: ExecEvent::NONE,
            jumped: false,
        }
    }

    fn dump_state(&mut self, ins: cpu::Instruction) {
        println!(
            "{:24} | {:5} | {:5} | {:5} | {:5} | {:5} | {:5} | {:5} | {}",
            ins.to_string(),
            self.pc,
            self.registers[0],
            self.registers[1],
            self.registers[2],
            self.registers[3],
            self.registers[4],
            self.registers[5],
            self.last_event.to_string()
        )
    }

    fn exec_next(&mut self) {
        let cur = self.get_mem(self.pc);

        self.jumped = false;
        self.last_event = ExecEvent::NONE;

        let ins = cpu::Instruction::decode(cur);
        ins.execute(self);
        
        if !self.jumped {
            self.pc += 1;
        }
        
        self.dump_state(ins);
    }

    pub fn run(&mut self) {
        println!("INS                      |    PC |    R1 |    R2 |    R3 |    R4 |    LP |    SP | EVENT");
        println!("=========================|=======|=======|=======|=======|=======|=======|=======|=============");
        while self.mem[0xffff] != 0 {
            self.exec_next();
        }
    }
}
