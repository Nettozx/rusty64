use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

const NUM_GPR: usize = 32; //number of general purpose registers

struct Cpu {
    //section 1.4.2 CPU Registers on datasheet
    reg_gpr: [u64; NUM_GPR],
    reg_fpr: [f64; NUM_GPR],

    reg_pc: u64,

    reg_hi: u64,
    reg_lo: u64,

    reg_llbit: bool, //TODO: Enum type

    reg_fcr0: u32,
    reg_fcr31: u32,

    cp0: Cp0
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {

            //TODO
            reg_gpr: [0; NUM_GPR],
            reg_fpr: [0.0; NUM_GPR],
            reg_pc: 0,
            reg_hi: 0,
            reg_lo: 0,
            reg_llbit: false,
            reg_fcr0: 0,
            reg_fcr31: 0,
            cp0: Cp0::new(),
        }
    }

    fn power_on_reset(&mut self) {
        self.cp0.power_on_reset();
    }
}

struct Cp0 {
    //page 46 on datasheet
    reg_index: u64,
    reg_random: u64,
    reg_entry_lo0: u64,
    reg_entry_lo1: u64,
    reg_context: u64,
    reg_page_mask: u64,
    reg_wired: u64,
    reg_bad_v_addr: u64,
    reg_count: u64,
    reg_entry_hi: u64,
    reg_compare: u64,
    reg_status: u64,
    reg_cause: u64,
    reg_epc: u64,
    reg_pr_id: u64,
    reg_config: u64,
    reg_ll_addr: u64,
    reg_watch_lo: u64,
    reg_watch_hi: u64,
    reg_x_context: u64,
    reg_tag_lo: u64,
    reg_tag_hi: u64,
    reg_error_epc: u64
}

impl Cp0 {
    fn new() -> Cp0 {
        Cp0 {

            //TODO
            reg_index: 0,
            reg_random: 0,
            reg_entry_lo0: 0,
            reg_entry_lo1: 0,
            reg_context: 0,
            reg_page_mask: 0,
            reg_wired: 0,
            reg_bad_v_addr: 0,
            reg_count: 0,
            reg_entry_hi: 0,
            reg_compare: 0,
            reg_status: 0,
            reg_cause: 0,
            reg_epc: 0,
            reg_pr_id: 0,
            reg_config: 0,
            reg_ll_addr: 0,
            reg_watch_lo: 0,
            reg_watch_hi: 0,
            reg_x_context: 0,
            reg_tag_lo: 0,
            reg_tag_hi: 0,
            reg_error_epc: 0,
        }
    }

    fn power_on_reset(&mut self) {
        //section 9.2.1 in datasheet & page 153
        //TODO

    }
}

fn main() {
    //N64 Specs
    //64-bit NEC VR4300 @93.75MHz
    //N64 Boot process
    //-On power N64 begins executing IPL(Initial Program Load) boot ROM at physical address
    // 0x1fc00000 (kseg 1 address 0xbfc00000). This initializes the hardware, does a few checks
    // and copies the cartridge boot code (cart offset 0x40-0x1000) to 0xa4000040 (RSP DMEM)
    // and jumps to it.
    //-After this cartridge boot code is in control. A portion of the main program code (starting
    // at cart offset 0x1000) is DMA'ed to the 4 byte address specified at the cart offset 0x8
    // (in the cart header). This address is reffered to as the Boot address offset or Program
    // Counter. The address is where the actual program will begin. Before this happems a CRC check
    // is done on the cartridge boot code, usually done somewhere around RDRAM address 0x80000100 -
    // 0x80000200. The results are compared to the two CRC values in the cart ROM's header. If they
    // don't match the N64 will hang in infinite loop using BGEZAL instruction.
    //-Note that most emulators choose not to boot from real PIF ROM and instead begin execution
    // at 0xa4000040 (after copying cart code there). Of course, they setup all the registers to the
    // values that the PIF ROM would put them in first.
    //-Also note that cartridge ROM is mapped at physical address 0x10000000 (kseg 1 address
    // 0xb0000000).

    //Now lets open a .n64 ROM file in hex mode and go to address 0x40, going to open
    // Legend of Zelda:Ocarina of Time.
    //To translate this we need to find out the instruction set so look at the datasheet
    // for this processor
    //http://datasheets.chipdb.org/NEC/Vr-Series/Vr43xx/U10504EJ7V0UMJ1.pdf
    //Look at detailed CPU Instruction set
    //We have to take the hex at address 0x40, which starts with a003 2048 in my case, on the ROM
    // and convert it to binary and match it up with the instruction set values to see what they do.
    //We can use MIPS Reference Data Card("Green Card") MAKE SURE YOU'RE READING IN BIG ENDIAN!
    //or onlinedisassembler.com and set it to mips:4300 (make sure your rom is not byte swapped,
    // you should be able to read the rom name on the side when looking at ROM data in hex mode).

    //setup arguments
    //pif rom will be the first argument
    let pif_file_name = env::args().nth(1).unwrap();
    //the rom file name is the second argument when running program
    let rom_file_name = env::args().nth(2).unwrap();

    //read pif rom and game rom
    let pif = read_bin(pif_file_name);
    let rom = read_bin(rom_file_name);

    //define cpu
    let mut cpu = Cpu::new();
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    //open the rom file
    let mut file = fs::File::open(path.as_ref()).unwrap();
    //read the rom
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    //get unmutable version of file_buf
    //in c++ everything is mutable by default, not in rust
    let file_buf = file_buf;
}