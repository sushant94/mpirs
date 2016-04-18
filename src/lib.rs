//! mpirs - An MPI Implementation in rust

extern crate rustc_serialize;
extern crate libc;

pub mod mpi_datatype;
pub mod mpi_comm;
pub mod mpi_request;
pub mod comm_request;
pub mod receiver_traits;

pub mod init;
pub mod finalize;
pub mod send;
pub mod wait;
pub mod receive;
pub mod bcast;
pub mod comm_rank;
pub mod num_procs;
pub mod scatter;
pub mod barrier;
pub mod gather;

pub mod utils {
    use libc;
    use std::io::Read;
    
    pub fn pid() -> u32 {
        unsafe { libc::getpid()  as u32 }
    }

    pub fn read_stream<T: Read>(stream: &mut T) -> String {
        let mut str_in = String::new();
        let mut bytes_read = [0; 2048];
        loop {
            let n = stream.read(&mut bytes_read).expect("Read Error:");
            str_in = format!("{}{}", str_in, String::from_utf8(bytes_read[0..n].to_vec()).unwrap());
            if n < 2048 {
                break;
            }
        }
        str_in
    }
}
