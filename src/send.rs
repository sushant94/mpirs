use rustc_serialize::base64;
use mpi_datatype::MPIDatatype;
use mpi_comm::MPIComm;
use mpi_request::MPIRequest;
use wait::mpi_wait;

// Functions in the Send module
pub fn mpi_isend<'a, T: AsRef<&'a [u8]>>(buf: T, count: u64, datatype: MPIDatatype,
			 dest: u64, tag: u64, comm: MPIComm, request: &mut MPIRequest) {

	unimplemented!();
	// let data = base64::to_base64(buf.as_ref());
}

pub fn mpi_send<'a, T: AsRef<&'a [u8]>>(buf: &T, count: u64, datatype: MPIDatatype,
			 dest: u64, tag: u64, comm: MPIComm) {

	let mut req: MPIRequest = Default::default();
	mpi_isend(buf, count, datatype, dest, tag, comm, &mut req);
	mpi_wait(&req);

}