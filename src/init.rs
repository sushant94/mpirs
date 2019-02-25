use libc;
use comm_request::CommRequest;
use comm_request::CommRequestType;
use comm_request::ControlTy;
use std::io::prelude::*;
use std::net::TcpStream;
use utils;

pub fn mpi_init() {
    let pid = utils::pid();
    let tag: u64 = u64::max_value();
    loop {
        if let Ok(ref mut stream) = TcpStream::connect("127.0.0.1:31337") {
            let commreq = CommRequest::<u32>::new(None,
                                                  None,
                                                  tag,
                                                  None,
                                                  CommRequestType::Control(ControlTy::Nop),
                                                  pid);

            let commreq_json = bincode::serialize(&commreq).unwrap();
            let _ = stream.write(&commreq_json);
            break;
        }
        unsafe {
            libc::usleep(1000);
        }
    }
}
