use serde::de::DeserializeOwned;
use mpi_comm::MPIComm;
use comm_request::RequestProc;
use std::fmt::Debug;
use serde::Serialize;
use send::mpi_send;
use receive::mpi_recv;
use comm_rank::mpi_comm_rank;
use num_procs::mpi_get_num_procs;

// Functions in the Scatter module
pub fn mpi_scatterv<T>(sendbuf: Vec<T>, sendcount: Vec<usize>,
			 displs: Vec<usize>, recvbuf: &mut T, root: usize, comm: MPIComm) 
			 where T: 'static + Debug + Clone + Serialize + DeserializeOwned + Send {

		let n = mpi_get_num_procs();
		let tag = u64::max_value();
		if mpi_comm_rank() == root {
			// verify if sum of sendcount equals size of sendbuf
			let count = sendcount.iter().fold(0, |mut sum, &x| {sum += x; sum});

			if count != sendbuf.len() {
				panic!("Sendcount does not add upto send buffer size");
			}

			for i in 0..n {
					let buf = sendbuf[displs[i]..sendcount[i]].to_vec();
			    mpi_send(&bincode::serialize(&buf).unwrap(), RequestProc::Process(i), tag, comm);
			}
		}
		
		mpi_recv(recvbuf, RequestProc::Process(root), tag, comm);
		
}

