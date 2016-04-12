use rustc_serialize::base64::{self, ToBase64}; 
use rustc_serialize::json;
use mpi_datatype::MPIDatatype;
use mpi_comm::MPIComm;
use mpi_request::MPIRequest;
use wait::mpi_wait;
use comm_request::CommRequest;
use comm_request::CommRequestType;

const MPI_RS: usize = 0;

// Functions in the Send module
pub fn mpi_isend<'a, T: AsRef<&'a [u8]>>(buf: T, count: u64, datatype: MPIDatatype,
			 dest: usize, tag: u64, comm: MPIComm, request: &mut MPIRequest) {

	let data = buf.as_ref().to_base64(base64::STANDARD);
	// src to be populated by mpi-rs master process
	let commReq: CommRequest = CommRequest::new(None, Some(dest), Some(tag), Some(data), CommRequestType::Message);
	let commReqJson = json::encode(&commReq).unwrap();
	// print JSON encoded request to stdout
	println!("{}", commReqJson);

	// construct MPIRequest
	*request = MPIRequest::new(MPI_RS, dest, tag);

}

pub fn mpi_send<'a, T: AsRef<&'a [u8]>>(buf: &T, count: u64, datatype: MPIDatatype,
			 dest: usize, tag: u64, comm: MPIComm) {

	let mut req: MPIRequest = Default::default();
	mpi_isend(buf, count, datatype, dest, tag, comm, &mut req);
	mpi_wait(&req);

}