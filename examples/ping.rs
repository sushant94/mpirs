extern crate mpirs;

use mpirs::{comm_rank, send, receive, mpi_datatype};
use mpirs::comm_request::RequestProc;
use mpirs::mpi_comm::MPI_COMM_WORLD;

const TAG: u64 = 42;

fn main() {
    let rank = comm_rank::mpi_comm_rank();

    if rank == 0 {
        let message = format!("ping!");
        send::mpi_send(&message,
                       message.len() as u64,
                       mpi_datatype::MPIDatatype::Int,
                       RequestProc::Process(1),
                       TAG,
                       MPI_COMM_WORLD);
        println!("Message Sent!");
    } else {
        let mut message = String::new();
        receive::mpi_recv(&mut message,
                          0,
                          mpi_datatype::MPIDatatype::Int,
                          RequestProc::Process(0),
                          TAG,
                          MPI_COMM_WORLD);
        println!("Hey! I got {}", message);
    }
}
