mod command;

use n64::*;
use n64::cpu::opcode::Opcode;
use n64::cpu::instruction::*;
use n64::mem_map::*;
use n64::mem_map::Addr::*;
use self::command::*;
use std::io::*;

pub struct Debugger {
    n64: N64,
    last_command: Option<Command>
}

impl Debugger {
    pub fn new(n64: N64) -> Debugger {
        Debugger {
            n64,
            last_command: None
        }
    }

    pub fn run(&mut self) {
        loop {
            print!("n64> ");
            stdout().flush().unwrap();
            let input = read_stdin();

            if let Ok(command) = input.parse() {
                if self.execute(command) {
                    break
                }
            } else { println!("Invalid input") }
        }
    }

    fn execute(&mut self, command: Command) -> bool {
        match command {
            Command::Step => self.step(),
            Command::Exit => return true,
            Command::Repeat => {
                if let Some(last_command) = self.last_command {
                    return self.execute(last_command)
                }
            }
        }
        if let Command::Repeat = command {} else { self.last_command = Some(command) };
        return false;
    }

    pub fn step(&mut self) {
        let current_pc = self.n64.cpu().current_pc_phys();
        let addr = map_addr(current_pc as u32);
        let instr = Instruction(match addr {
            PifRom(offset) => self.n64.interconnect().pif().read_boot_rom(offset),
            _ => panic!("Debugger can't inspect address: {:?}", addr)
        });
        print!("{:018X}: ", current_pc);
        match instr.opcode() {
            SPECIAL => print!("{:?}(SpecialOp)", instr.special_op()),
            REGIMM => print!("{:?}(RegImmOp)", instr.reg_imm_op()),
            _ => print!("{:?}", instr)
        }
        if self.n64.cpu().will_execute_from_delay_slot() {
            println!("(DelaySlot)") } else { println!() }
        self.n64.step()
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().into()
}