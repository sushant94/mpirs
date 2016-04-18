extern crate mpirs;
extern crate rustc_serialize;
extern crate libc;
extern crate stopwatch;
extern crate rand;

use stopwatch::Stopwatch;

use mpirs::{comm_rank, num_procs, send, receive, init, finalize};
use mpirs::comm_request::RequestProc;
use mpirs::mpi_comm::MPI_COMM_WORLD;
use mpirs::barrier;

use rand::distributions::{IndependentSample, Range};

const TAG: u64 = 42;
const SIZE: u64 = 1024;
const A: f32 = 313.37;

fn main() {
    unsafe {
        libc::sleep(4);
    }

    let rank = comm_rank::mpi_comm_rank();
    let size = num_procs::mpi_get_num_procs();

    let between = Range::new(0f32, 8192f32);
    let mut rng = rand::thread_rng();

    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();
    let mut result: f32 = 0.0;

    for _ in 0..SIZE {
        vec_a.push(between.ind_sample(&mut rng));
        vec_b.push(between.ind_sample(&mut rng));
    }

    barrier::mpi_barrier();
    let sw = Stopwatch::start_new();

    for (i, j) in vec_a.iter().zip(vec_b) {
        result += A * i + j;
    }

    if rank == 0 {
        let mut value: f32 = 0.0;
        for _ in 0..size - 1 {
            receive::mpi_recv(&mut value,
                              RequestProc::Any,
                              TAG,
                              MPI_COMM_WORLD);
        }
    } else {
        send::mpi_send(&result, RequestProc::Process(0), TAG, MPI_COMM_WORLD);
    }

    barrier::mpi_barrier();

    if rank == 0 {
        print!("{}", (sw.elapsed_ms() as f64) / (1000 as f64));
    }

    finalize::mpi_finalize();
}
