use rustc_serialize::Decodable;
use mpi_datatype::MPIDatatype;
use mpi_comm::MPIComm;
use comm_request::CommRequest;
use std::sync::mpsc::{Sender, Receiver};
use receiver_traits::Message;

// Functions in the Receive module
pub fn mpi_irecv<T: Decodable>(buf: &mut T, count: u64, datatype: MPIDatatype,
			 dest: usize, tag: u64, comm: MPIComm) -> Receiver<CommRequest> {

	unimplemented!();
}

pub fn mpi_recv<T: Decodable>(buf: &mut T, count: u64, datatype: MPIDatatype,
			 dest: usize, tag: u64, comm: MPIComm) {
	
	let mut rx: Receiver<CommRequest> = mpi_irecv(buf, count, datatype, dest, tag, comm);
	*buf = rx.wait();
}