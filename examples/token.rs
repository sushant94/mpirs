extern crate mpirs;
extern crate rustc_serialize;
extern crate libc;

use mpirs::{comm_rank, num_procs, send, receive, mpi_datatype};
use mpirs::comm_request::RequestProc;
use mpirs::mpi_comm::MPI_COMM_WORLD;

const TAG: u64 = 42;

fn main() {
    unsafe {
        libc::sleep(4);
    }
    let rank = comm_rank::mpi_comm_rank();
    let size = num_procs::mpi_get_num_procs();

    if rank == 0 {
        let mut token = format!("{}", rank);
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
        println!("Process {} received token {:?} from process {}",
                 rank,
                 token,
                 size - 1);
    } else {
        let mut token: String = String::new();
        receive::mpi_recv(&mut token,
                          1,
                          mpi_datatype::MPIDatatype::Int,
                          RequestProc::Process(rank - 1),
                          TAG,
                          MPI_COMM_WORLD);
        println!("Process {} received token {:?} from process {}",
                 rank,
                 token,
                 rank - 1);

        token = format!("{}-{}", token, rank);
        send::mpi_send(&token,
                       1,
                       mpi_datatype::MPIDatatype::Int,
                       RequestProc::Process((rank + 1) % size),
                       TAG,
                       MPI_COMM_WORLD);
    }
}
