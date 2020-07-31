use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref HOST: Mutex<SerialPort> = {
        let standard_interface_port = 0x3F8;
        let mut serial_port = unsafe {SerialPort::new(standard_interface_port)};
        serial_port.init();
        Mutex::new(serial_port)
    };
}