extern crate mpirs;

use mpirs::{comm_rank};

fn main() {
    let rank = comm_rank::mpi_comm_rank();
    println!("rank: {}", rank);
}
