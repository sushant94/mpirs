use rustc_serialize::json;
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

// Functions in the Scatter module
pub fn mpi_scatterv<T>(sendbuf: Vec<T>, sendcount: Vec<usize>,
			 displs: Vec<usize>, datatype: MPIDatatype, recvbuf: &mut T, recvcount: u64, root: usize, comm: MPIComm) 
			 where T: 'static + Debug + Clone + Encodable + Decodable + Send {

		let n = 4;
		let tag = 42;
		if mpi_comm_rank() == root {

			// verify if sum of sendcount equals size of sendbuf
			let count = sendcount.iter().fold(0, |mut sum, &x| {sum += x; sum});

			if count != sendbuf.len() {
				panic!("Sendcount does not add upto send buffer size");
			}

			for i in 0..n {
					let mut buf = sendbuf[displs[i]..sendcount[i]].to_vec();
			    mpi_send(&json::encode(&buf).unwrap(), buf.len() as u64, datatype, RequestProc::Process(i), tag, comm);
			}
		}
		
		mpi_recv(recvbuf, recvcount, datatype, RequestProc::Process(root), tag, comm);
		
}

