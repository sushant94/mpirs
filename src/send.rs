use rustc_serialize::json::{self, ToJson};
use mpi_datatype::MPIDatatype;
use mpi_comm::MPIComm;
use comm_request::CommRequest;
use comm_request::CommRequestType;
use std::sync::mpsc::{Sender, Receiver};
use receiver_traits::Message;

const MPI_RS: usize = 0;

// Functions in the Send module
pub fn mpi_isend<T: ToJson>(buf: T, count: u64, datatype: MPIDatatype,
			 dest: usize, tag: u64, comm: MPIComm) -> Receiver<CommRequest> {

	unimplemented!();
}

pub fn mpi_send<T: ToJson>(buf: T, count: u64, datatype: MPIDatatype,
			 dest: usize, tag: u64, comm: MPIComm) {

	let mut rx: Receiver<CommRequest> = mpi_isend(buf, count, datatype, dest, tag, comm);
	rx.wait();

}