mod audio_interface;
pub mod cpu;
mod interconnect;
pub mod mem_map;
mod n64;
mod peripheral_interface;
mod pif;
mod rdp;
mod rsp;
mod serial_interface;
mod video_interface;

pub use self::cpu::Cpu;
pub use self::interconnect::Interconnect;
pub use self::n64::N64;
pub use self::pif::Pif;
type AudioInterface = self::audio_interface::AudioInterface;
type PeripheralInterface = self::peripheral_interface::PeripheralInterface;
type Rsp = self::rsp::Rsp;
type SerialInterface = self::serial_interface::SerialInterface;
type VideoInterface = self::video_interface::VideoInterface;