//! Implements mpi_barrier
use rustc_serialize::json;
use comm_request::CommRequest;
use comm_request::CommRequestType;
use comm_request::ControlTy;
use std::io::prelude::*;
use std::net::TcpStream;
use utils;

pub fn mpi_barrier() {
    let pid = utils::pid();
    let tag = u64::max_value();
    let commreq = CommRequest::<u32>::new(None,
                                          None,
                                          tag,
                                          None,
                                          CommRequestType::Control(ControlTy::Barrier),
                                          pid);

    let commreq_json = json::encode(&commreq).expect("Cannot encode to json");
    let mut stream = TcpStream::connect("127.0.0.1:31337").unwrap();
    let _ = stream.write(&commreq_json.as_bytes());

    // Discard the ACK
    let _ = utils::read_stream(&mut stream);
}
