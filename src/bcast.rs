use rustc_serialize::Decodable;
use mpi_datatype::MPIDatatype;
use mpi_comm::MPIComm;
use comm_request::CommRequest;
use comm_request::RequestProc;
use std::fmt::Debug;
use rustc_serialize::Encodable;
use send::mpi_send;
use receive::mpi_recv;
use comm_rank::mpi_comm_rank;

// Functions in the Broadcast module
pub fn mpi_bcast<T: Clone + Debug + Decodable + Encodable>(buf: &mut T, count: u64, datatype: MPIDatatype,
			 root: usize, comm: MPIComm) {

		let n = 4;
		let tag = 42;
		if mpi_comm_rank() == root {
			for i in 0..n {
			    mpi_send(buf, count, datatype, RequestProc::Any, tag, comm);
			} 
		} else {
			    mpi_recv(buf, count, datatype, RequestProc::Process(root), tag, comm);
		}
		
}

