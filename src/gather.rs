use serde::de::DeserializeOwned;
use mpi_comm::MPIComm;
use comm_request::RequestProc;
use std::fmt::Debug;
use serde::Serialize;
use send::mpi_send;
use receive::mpi_recv;
use comm_rank::mpi_comm_rank;
use num_procs::mpi_get_num_procs;
use std::u64;

// Functions in the Gather module
pub fn mpi_gatherv<T>(sendbuf: &mut T, recvbuf: &mut Vec<T>, 
			 recvcount: Vec<usize>, displs: Vec<usize>, root: usize, comm: MPIComm) 
			 where T: 'static + Debug + Clone + Serialize + DeserializeOwned + Send {

		let n = mpi_get_num_procs();
		let tag = u64::MAX;
		if mpi_comm_rank() == root {

			// find total recv buffer size
			let count = recvcount.iter().fold(0, |mut sum, &x| {sum += x; sum});
			recvbuf.reserve(count);

			for i in 0..n {
					let mut buf_str: String = String::new();
			    mpi_recv(&mut buf_str, RequestProc::Process(i), tag, comm);
			    let buf: Vec<T> = bincode::deserialize(buf_str.as_bytes()).unwrap();

			    if buf.len() != recvcount[i] {
			    	panic!("Received more than specified buffer size");
			    }

			    for j in 0..buf.len() {
			    	recvbuf[displs[i]+j] = buf[j].clone();
			    }
			}
		}
		
		mpi_send(sendbuf, RequestProc::Process(root), tag, comm);
		
}
