extern crate mpirs;
extern crate rustc_serialize;
extern crate libc;

use std::fmt;

use mpirs::{comm_rank, num_procs, send, receive, mpi_datatype, init, finalize, bcast};
use mpirs::comm_request::RequestProc;
use mpirs::mpi_comm::MPI_COMM_WORLD;

const TAG: u64 = 42;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
struct Token {
    pub val: u64,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

fn main() {
    unsafe {
        libc::usleep(32000);
    }

    let rank = comm_rank::mpi_comm_rank();
    let size = num_procs::mpi_get_num_procs();
    let mut token = Token { val: 65 };

    bcast::mpi_bcast(&mut token, 1, mpi_datatype::MPIDatatype::Int, 0, MPI_COMM_WORLD);

    println!("Process {} has token {}", rank, token);
    finalize::mpi_finalize();
}
