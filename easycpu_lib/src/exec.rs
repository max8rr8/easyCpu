use std::{fmt::Debug, mem::swap, ops::AddAssign};

use crate::cpu;

#[derive(Debug, Clone)]
pub enum ExecEvent {
    NONE,
    JUMP(u16),
    REGGET(cpu::Register, u16),
    REGSET(cpu::Register, u16),
    MEMGET(u16, u16),
    MEMSET(u16, u16),
}

#[derive(Clone, Debug, Default)]
pub struct ExecStats {
    pub nop: usize,
    pub branch: usize,
    pub store: usize,
    pub load: usize,
    pub alu: usize,
}

impl AddAssign<&Self> for ExecStats {
    fn add_assign(&mut self, rhs: &Self) {
        self.nop += rhs.nop;
        self.branch += rhs.branch;
        self.store += rhs.store;
        self.load += rhs.load;
        self.alu += rhs.alu;
    }
}

#[derive(Clone, Debug)]
pub struct ExecCpu {
    pc: u16,
    registers: [u16; 6],
    mem: Vec<u16>,
    events: Vec<ExecEvent>,
    jumped: bool,
    stats: ExecStats,
}

impl ExecCpu {
    pub fn get_reg(&mut self, reg: crate::cpu::Register) -> u16 {
        let val = match reg {
            cpu::Register::ZX => 0,
            cpu::Register::PC => self.pc,
            cpu::Register::R2 => self.registers[0],
            cpu::Register::R3 => self.registers[1],
            cpu::Register::R4 => self.registers[2],
            cpu::Register::R5 => self.registers[3],
            cpu::Register::LP => self.registers[4],
            cpu::Register::SP => self.registers[5],
        };
        self.events.push(ExecEvent::REGGET(reg, val));
        val
    }

    pub fn set_reg(&mut self, reg: crate::cpu::Register, val: u16) {
        if reg != cpu::Register::PC {
            self.events.push(ExecEvent::REGSET(reg, val));
        }

        match reg {
            cpu::Register::ZX => (),
            cpu::Register::PC => {
                self.jumped = true;
                self.events.push(ExecEvent::JUMP(val));
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

    pub fn get_mem(&mut self, addr: u16) -> u16 {
        let val = *self.mem.get(addr as usize).unwrap_or(&0);
        self.events.push(ExecEvent::MEMGET(addr, val));

        val
    }

    pub fn set_mem(&mut self, addr: u16, val: u16) {
        self.events.push(ExecEvent::MEMSET(addr, val));
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
            events: Vec::new(),
            jumped: false,
            stats: Default::default(),
        }
    }

    pub fn reset_stats(&mut self) {
        self.stats = Default::default();
    }

    pub fn get_stats(&self) -> &ExecStats {
        &self.stats
    }

    pub fn exec_next(&mut self) -> (cpu::Instruction, Vec<ExecEvent>) {
        let cur = self.get_mem(self.pc);

        self.jumped = false;
        self.events.clear();

        let ins = cpu::Instruction::decode(cur);

        match ins {
            cpu::Instruction::NOP => self.stats.nop += 1,
            cpu::Instruction::AND(_) | cpu::Instruction::ADD(_) => self.stats.alu += 1,
            cpu::Instruction::LOAD(_) => self.stats.load += 1,
            cpu::Instruction::STORE(_) => self.stats.store += 1,
            cpu::Instruction::BRANCH(_) => self.stats.branch += 1,
            cpu::Instruction::CUSTOM(_) => self.stats.nop += 1,
        }

        ins.execute(self);

        if !self.jumped {
            self.pc += 1;
        }

        let mut events = Vec::new();
        swap(&mut events, &mut self.events);

        (ins, events)
    }
}
