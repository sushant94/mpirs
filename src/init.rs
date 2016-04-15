use libc;
use std::net::TcpStream;

pub fn mpi_init() {
	  loop {
				if let Ok(_) = TcpStream::connect("127.0.0.1:31337") {
					break;
				}
				unsafe { 
					libc::usleep(1000);
				}
		}
	
}