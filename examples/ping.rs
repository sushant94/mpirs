extern crate mpirs;

use mpirs::{comm_rank, send, receive, finalize};
use mpirs::comm_request::RequestProc;
use mpirs::mpi_comm::MPI_COMM_WORLD;

const TAG: u64 = 42;

fn main() {
    let rank = comm_rank::mpi_comm_rank();
    if rank == 0 {
        let mut message = format!("ping!");
        send::mpi_send(&message,
                       RequestProc::Process(1),
                       TAG,
                       MPI_COMM_WORLD);
        println!("Ping Sent!");
        message = String::new();
        receive::mpi_recv(&mut message,
                          RequestProc::Process(1),
                          TAG,
                          MPI_COMM_WORLD);
        println!("Hey! I got {}", message);
    } else {
        let mut message = String::new();
        receive::mpi_recv(&mut message,
                          RequestProc::Process(0),
                          TAG,
                          MPI_COMM_WORLD);
        println!("Hey! I got {}", message);
        message = "pong".to_owned();
        send::mpi_send(&message,
                       RequestProc::Process(0),
                       TAG,
                       MPI_COMM_WORLD);
        println!("Pong sent!");
    }
    finalize::mpi_finalize();
}
