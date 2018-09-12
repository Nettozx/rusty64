use n64::*;

pub struct Debugger {
    n64: N64
}

impl Debugger {
    pub fn new(n64: N64) -> Debugger {
        Debugger {
            n64
        }
    }

    pub fn run(&mut self) {
        loop {
            self.n64.run_instruction();
        }
    }
}