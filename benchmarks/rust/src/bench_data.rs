extern crate mpirs;
extern crate rand;
extern crate libc;
extern crate stopwatch;

use stopwatch::Stopwatch;

use mpirs::{comm_rank, send, receive, finalize, barrier};
use mpirs::comm_request::RequestProc;
use mpirs::mpi_comm::MPI_COMM_WORLD;

const TAG: u64 = 42;
const SIZE: u64 = 16384;

fn main() {
    unsafe {
        libc::sleep(4);
    }
    let rank = comm_rank::mpi_comm_rank();

    let mut data = Vec::new();
    if rank == 0 {
        for _ in 0..SIZE + 1 {
            data.push(rand::random::<u32>());
        }
    }

    barrier::mpi_barrier();
    let sw = Stopwatch::start_new();

    if rank == 0 {
        send::mpi_send(&data,
                       RequestProc::Process(1),
                       TAG,
                       MPI_COMM_WORLD);
    } else {
        receive::mpi_recv(&mut data,
                          RequestProc::Process(0),
                          TAG,
                          MPI_COMM_WORLD);
    }

    barrier::mpi_barrier();
    if rank == 0 {
        print!("{}", (sw.elapsed_ms() as f64) / (1000 as f64));
    }

    finalize::mpi_finalize();
}
