extern crate mpirs;
extern crate rustc_serialize;
extern crate libc;
extern crate stopwatch;

use stopwatch::Stopwatch;

use mpirs::{comm_rank, num_procs, send, receive, init, finalize};
use mpirs::comm_request::RequestProc;
use mpirs::mpi_comm::MPI_COMM_WORLD;
use mpirs::barrier;

const TAG: u64 = 42;
fn main() {
    unsafe {
        libc::sleep(4);
    }

    let rank = comm_rank::mpi_comm_rank();
    let size = num_procs::mpi_get_num_procs();
    barrier::mpi_barrier();
    let sw = Stopwatch::start_new();

    if rank == 0 {
        let mut token = 65;
        send::mpi_send(&token,
                       RequestProc::Process((rank + 1) % size),
                       TAG,
                       MPI_COMM_WORLD);

        receive::mpi_recv(&mut token,
                          RequestProc::Process(size - 1),
                          TAG,
                          MPI_COMM_WORLD);
    } else {
        let mut token: u64 = 0;
        receive::mpi_recv(&mut token,
                          RequestProc::Process(rank - 1),
                          TAG,
                          MPI_COMM_WORLD);
        token += 1;
        send::mpi_send(&token,
                       RequestProc::Process((rank + 1) % size),
                       TAG,
                       MPI_COMM_WORLD);
    }


    barrier::mpi_barrier();

    if rank == 0 {
        print!("{}", (sw.elapsed_ms() as f64) / (1000 as f64));
    }

    finalize::mpi_finalize();
}
