use rustc_serialize::Decodable;
use mpi_comm::MPIComm;
use comm_request::RequestProc;
use std::fmt::Debug;
use rustc_serialize::Encodable;
use send::mpi_send;
use receive::mpi_recv;
use comm_rank::mpi_comm_rank;
use num_procs::mpi_get_num_procs;

// Functions in the Broadcast module
pub fn mpi_bcast<T>(buf: &mut T, root: usize, comm: MPIComm)
    where T: 'static + Debug + Clone + Encodable + Decodable + Send
{
    let n = mpi_get_num_procs();;
    let tag = u64::max_value();
    if mpi_comm_rank() == root {
        for _ in 0..n - 1 {
            mpi_send(buf, RequestProc::Any, tag, comm);
        }
    } else {
        mpi_recv(buf, RequestProc::Process(root), tag, comm);
    }
}
