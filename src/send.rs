use rustc_serialize::json::{self, ToJson};
use mpi_datatype::MPIDatatype;
use mpi_comm::MPIComm;
use comm_request::CommRequest;
use comm_request::CommRequestType;
use comm_request::RequestProc;
use std::sync::mpsc::{Sender, Receiver};
use receiver_traits::Message;
use std::fmt::Debug;
use rustc_serialize::Encodable;

const MPI_RS: usize = 0;

// Functions in the Send module
pub fn mpi_isend<T: Debug + Clone + Encodable>(buf: &T, count: u64, datatype: MPIDatatype,
			 dest: RequestProc, tag: u64, comm: MPIComm) -> Receiver<CommRequest<T>> {

	unimplemented!();
}

pub fn mpi_send<T: Debug + Clone + Encodable>(buf: &T, count: u64, datatype: MPIDatatype,
			 dest: RequestProc, tag: u64, comm: MPIComm) {

	let mut rx: Receiver<CommRequest<T>> = mpi_isend(buf, count, datatype, dest, tag, comm);
	rx.wait();

}
