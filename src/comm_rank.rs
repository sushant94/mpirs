use rustc_serialize::json;
use libc;
use comm_request::CommRequest;
use comm_request::CommRequestType;
use comm_request::ControlTy;
use std::io::prelude::*;
use std::net::TcpStream;
use std::u64;

pub fn mpi_comm_rank() -> usize {
	  let pid = unsafe { libc::getpid() } as u32;
	  let tag:u64 = u64::MAX;
	  let mut rank:Option<usize> = None;
		let commreq = CommRequest::<u32>::new(None, None, tag, None, CommRequestType::Control(ControlTy::GetMyRank), pid);
		let commreq_json = json::encode(&commreq).unwrap();
		
		let mut stream = TcpStream::connect("127.0.0.1:31337").unwrap();
		let _ = stream.write(&commreq_json.as_bytes());
		let mut bytes_read = [0; 2048];
		let mut str_in = String::new();

		loop {
		    let n = stream.read(&mut bytes_read).expect("Read Error:");
		    str_in = format!("{}{}", str_in,
		    								String::from_utf8(bytes_read[0..n].to_vec()).unwrap());

		    if n < 2048 {
		    	break;
		    } 
		}

		if !str_in.is_empty() {
		  rank = usize::from_str_radix(&str_in, 10).ok();
		}

		rank.expect("Rank fetching failed")
}