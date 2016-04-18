extern crate mpirs;
extern crate rustc_serialize;
extern crate libc;

use std::fmt;

use mpirs::{comm_rank, init, finalize, bcast};
use mpirs::mpi_comm::MPI_COMM_WORLD;

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
    let mut token = Token { val: 65 };

    bcast::mpi_bcast(&mut token, 1, MPI_COMM_WORLD);

    println!("Process {} has token {}", rank, token);
    finalize::mpi_finalize();
}
