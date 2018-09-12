mod command;

use n64::*;
use self::command::*;
use std::io::*;

pub struct Debugger {
    n64: N64
}

impl Debugger {
    pub fn new(n64: N64) -> Debugger {
        Debugger {
            n64: n64
        }
    }

    pub fn run(&mut self) {
        loop {
            print!("n64> ");
            stdout().flush().unwrap();
            let input = read_stdin();

            let command = input.parse();
            match command {
                Ok(Command::Step) => self.step(),
                Ok(Command::Exit) => break,
                Err(_) => println!("Invalid input")
            }
            //self.n64.run_instruction();
        }
    }

    pub fn step(&mut self) {
        //Print next PC/instr
        println!("{:018X}", self.n64.cpu().reg_pc());
//        match instr.opcode() {
//            SPECIAL => print!("{:?}(SpecialOp)", instr.special_op()),
//            REGIMM => print!("{:?}(RegImmOp)", instr.reg_imm_op()),
//            _ => print!("{:?}", instr)
//        }
//        if let DelaySlot::Yes = delay_slot { println!("(DelaySlot)") } else { println!() }
        self.n64.step()
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().into()
}