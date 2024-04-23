use wasm_bindgen::prelude::*;
use easycpu_lib::{cpu, exec::ExecCpu};

#[wasm_bindgen]
pub struct RegistersState {
    pub pc: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub sp: u16,
    pub lp: u16,
}

#[wasm_bindgen]
pub struct DebugCpu {
    cpu: ExecCpu
}

#[wasm_bindgen]
impl DebugCpu {
    
    #[wasm_bindgen(constructor)]
    pub fn new(init_ram: Vec<u16>)  -> Self {
        Self {
            cpu: ExecCpu::new(init_ram)
        } 
    }

    pub fn reset(&mut self, init_ram: Vec<u16>) {
        self.cpu = ExecCpu::new(init_ram)
    }

    pub fn get_registers(&mut self) -> RegistersState {
        RegistersState {
            pc: self.cpu.get_reg(cpu::Register::PC),
            r2: self.cpu.get_reg(cpu::Register::R2),
            r3: self.cpu.get_reg(cpu::Register::R3),
            r4: self.cpu.get_reg(cpu::Register::R4),
            r5: self.cpu.get_reg(cpu::Register::R5),
            sp: self.cpu.get_reg(cpu::Register::SP),
            lp: self.cpu.get_reg(cpu::Register::LP),
        }
    }

    pub fn set_register(&mut self, reg: u16, val: u16) {
        self.cpu.set_reg(cpu::Register::from(reg), val)
    }
    
    pub fn step(&mut self) {
        self.cpu.exec_next();
    }

    pub fn keep_running(&mut self) -> bool {
        self.cpu.get_mem(0xffff) != 0
    }

    pub fn read_memory(&mut self, mut from: u16, to: u16) -> Vec<u16> {
        let mut res = Vec::new();
        
        while from != to {
            res.push(self.cpu.get_mem(from));
            from = from.wrapping_add(1)
        }

        res
    }

    pub fn write_memory(&mut self, addr: u16, val: u16) {
        self.cpu.set_mem(addr, val);
    }
}