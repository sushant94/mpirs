//! mpirs - An MPI Implementation in rust

extern crate rustc_serialize;

pub mod mpi_datatype;
pub mod mpi_comm;
pub mod mpi_request;
pub mod comm_request;

pub mod send;
pub mod wait;
pub mod receive;