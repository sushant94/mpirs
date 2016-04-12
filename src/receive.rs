use rustc_serialize::base64;
use mpi_datatype::MPIDatatype;
use mpi_comm::MPIComm;
use mpi_request::MPIRequest;
use std::sync::Arc;

// Functions in the Receive module
pub fn mpi_irecv<'a, T: AsRef<&'a [u8]>>(count: u64, datatype: MPIDatatype,
			 dest: u64, tag: u64, comm: MPIComm, request: &mut MPIRequest) -> Arc<T> {

	unimplemented!();
	// let data = base64::from_base64();
}

pub fn mpi_recv<'a, T: AsRef<&'a [u8]>>(count: u64, datatype: MPIDatatype,
			 dest: u64, tag: u64, comm: MPIComm) -> Arc<T> {
	unimplemented!();
}