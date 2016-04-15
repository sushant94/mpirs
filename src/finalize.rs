use rustc_serialize::json::{self, ToJson};
use libc;
use mpi_datatype::MPIDatatype;
use mpi_comm::MPIComm;
use comm_request::CommRequest;
use comm_request::CommRequestType;
use comm_request::RequestProc;
use comm_request::ControlTy;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::thread;
use receiver_traits::Message;
use std::fmt::Debug;
use rustc_serialize::Encodable;
use rustc_serialize::Decodable;
use std::io::prelude::*;
use std::net::TcpStream;
use std::u64;

pub fn mpi_finalize() {
	  let pid = unsafe { libc::getpid() } as u32;
	  let tag:u64 = u64::MAX;
		let mut commreq = CommRequest::<u32>::new(None, None, tag, None, CommRequestType::Control(ControlTy::Exit), pid);
		let commreq_json = json::encode(&commreq).unwrap();
		
		let mut stream = TcpStream::connect("127.0.0.1:31337").unwrap();
		let _ = stream.write(&commreq_json.as_bytes());

}