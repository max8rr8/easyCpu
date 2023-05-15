use easycpu_lib::{exec::{ExecCpu, ExecEvent}, cpu::Register};

pub struct DebugCpu {
    cpu: ExecCpu,
}

impl DebugCpu {
    pub fn new(init_ram: Vec<u16>)  -> Self {
        Self {
            cpu: ExecCpu::new(init_ram)
        } 
    }

    fn format_event(event: ExecEvent) -> Option<String> {
        match event {
            ExecEvent::NONE => None,
            ExecEvent::JUMP(targ) => Some(format!("JUMP({:#06x})", targ)),
            ExecEvent::REGGET(_, _) => None,
            ExecEvent::REGSET(_, _) => None,
            ExecEvent::MEMGET(addr, val) => Some(format!("MEMGET({:#06x}) => {:#06x}", addr, val)),
            ExecEvent::MEMSET(addr, val) => Some(format!("MEMGET({:#06x}) => {:#06x}", addr, val)),
        }
    }
    
    pub fn run(&mut self) {
        println!("INS                      |     PC |     R1 |     R2 |     R3 |     R4 |     LP |     SP | EVENT");
        println!("=========================|========|========|========|========|========|========|========|=============");
        while self.cpu.get_mem(0xffff) != 0 {
            let (ins, events) = self.cpu.exec_next();

            let events_fmt = events.into_iter().filter_map(Self::format_event).fold(String::new(), |a, b| a + &b + "; ");

            println!(
                "{:24} | {:#06x} | {:#06x} | {:#06x} | {:#06x} | {:#06x} | {:#06x} | {:#06x} | {}",
                ins.to_string(),
                self.cpu.get_reg(Register::PC),
                self.cpu.get_reg(Register::R2),
                self.cpu.get_reg(Register::R3),
                self.cpu.get_reg(Register::R4),
                self.cpu.get_reg(Register::R5),
                self.cpu.get_reg(Register::LP),
                self.cpu.get_reg(Register::SP),
                events_fmt
            )
        }
    }

}
