extern crate mpirs;
extern crate rustc_serialize;
extern crate libc;

use std::fmt;

use mpirs::{comm_rank, num_procs, send, receive, mpi_datatype, init, finalize};
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

    if rank == 0 {
        let mut token = Token { val: 65 };
        send::mpi_send(&token,
                       1,
                       mpi_datatype::MPIDatatype::Int,
                       RequestProc::Process((rank + 1) % size),
                       TAG,
                       MPI_COMM_WORLD);

        receive::mpi_recv(&mut token,
                          1,
                          mpi_datatype::MPIDatatype::Int,
                          RequestProc::Process(size - 1),
                          TAG,
                          MPI_COMM_WORLD);
        println!("Process {} received token {} from process {}",
                 rank,
                 token,
                 size - 1);
    } else {
        let mut token = Token { val: 0 };
        receive::mpi_recv(&mut token,
                          1,
                          mpi_datatype::MPIDatatype::Int,
                          RequestProc::Process(rank - 1),
                          TAG,
                          MPI_COMM_WORLD);
        println!("Process {} received token {} from process {}",
                 rank,
                 token,
                 rank - 1);

        token.val += 1;
        send::mpi_send(&token,
                       1,
                       mpi_datatype::MPIDatatype::Int,
                       RequestProc::Process((rank + 1) % size),
                       TAG,
                       MPI_COMM_WORLD);
    }

    finalize::mpi_finalize();
}
