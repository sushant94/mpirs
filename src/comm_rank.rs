use comm_request::CommRequest;
use comm_request::CommRequestType;
use comm_request::ControlTy;
use std::io::prelude::*;
use std::net::TcpStream;
use utils;

pub fn mpi_comm_rank() -> usize {
    let pid = utils::pid();
    let tag: u64 = u64::max_value();
    let mut rank: Option<usize> = None;
    let commreq = CommRequest::<u32>::new(None,
                                          None,
                                          tag,
                                          None,
                                          CommRequestType::Control(ControlTy::GetMyRank),
                                          pid);
    let commreq_json = bincode::serialize(&commreq).unwrap();

    let mut stream = TcpStream::connect("127.0.0.1:31337").unwrap();
    let _ = stream.write(&commreq_json);

    let mut str_in = utils::read_stream(&mut stream);
    if !str_in.is_empty() {
        rank = usize::from_str_radix(&str_in, 10).ok();
    }

    rank.expect("Rank fetching failed")
}
