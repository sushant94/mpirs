//! mpirs - An MPI Implementation in rust

extern crate rustc_serialize;
extern crate libc;

pub mod mpi_datatype;
pub mod mpi_comm;
pub mod mpi_request;
pub mod comm_request;
pub mod receiver_traits;

pub mod init;
pub mod finalize;
pub mod send;
pub mod wait;
pub mod receive;
pub mod bcast;
pub mod comm_rank;
pub mod num_procs;
pub mod scatter;
pub mod gather;