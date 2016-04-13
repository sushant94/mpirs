use rustc_serialize::Decodable;
use mpi_datatype::MPIDatatype;
use mpi_comm::MPIComm;
use comm_request::CommRequest;
use std::sync::mpsc::{Sender, Receiver};
use receiver_traits::Message;
use std::fmt::Debug;
use rustc_serialize::Encodable;

// Functions in the Receive module
pub fn mpi_irecv<T: Clone + Debug + Decodable + Encodable>(buf: &mut T, count: u64, datatype: MPIDatatype,
			 dest: usize, tag: u64, comm: MPIComm) -> Receiver<CommRequest<T>> {

	unimplemented!();
}

pub fn mpi_recv<T: Clone + Debug + Decodable + Encodable>(buf: &mut T, count: u64, datatype: MPIDatatype,
			 dest: usize, tag: u64, comm: MPIComm) {
	
	let mut rx: Receiver<CommRequest<T>> = mpi_irecv(buf, count, datatype, dest, tag, comm);
	*buf = rx.wait();
}
