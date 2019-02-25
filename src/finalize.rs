use comm_request::CommRequest;
use comm_request::CommRequestType;
use comm_request::ControlTy;
use std::io::prelude::*;
use std::net::TcpStream;
use utils;

pub fn mpi_finalize() {
    let pid = utils::pid();
    let tag: u64 = u64::max_value();
    let commreq = CommRequest::<u32>::new(None,
                                          None,
                                          tag,
                                          None,
                                          CommRequestType::Control(ControlTy::Exit),
                                          pid);
    let commreq_serialized = bincode::serialize(&commreq).unwrap();

    let mut stream = TcpStream::connect("127.0.0.1:31337").unwrap();
    let _ = stream.write(&commreq_serialized);
}
