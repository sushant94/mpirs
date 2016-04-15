//! Implements mpirun equivalent
//!
//! ## Usage
//! `mpirun -n <num_of_procs> /path/to/executable`
//!
//! ## Implementation details.
//! mpirun spawns
//!
//!

extern crate docopt;
extern crate rustc_serialize;
extern crate mpirs;

mod mailbox;

use std::process::{Command, Stdio};
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

use rustc_serialize::json;

use docopt::Docopt;

use mpirs::comm_request::{CommRequest, CommRequestType, ControlTy, RequestProc};
use mailbox::Mailbox;

static USAGE: &'static str = "
mpirs. Run MPI Programs in rust.
Usage:
  mpirs [options] \
                              [<executable>]

Options:
  -n --num=<num_of_procs>   Define the \
                              number of processes to spawn
  -h --help                 Show this \
                              help screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_executable: String,
    flag_num: Option<usize>,
}

fn read_from_stream(stream: &mut TcpStream) -> String {
    let mut bytes_read = [0; 2048];
    let mut str_in = String::new();
    loop {
        let n = stream.read(&mut bytes_read).expect("Read Error:");
        str_in = format!("{}{}",
                         str_in,
                         String::from_utf8(bytes_read[0..n].to_vec()).unwrap());
        if n < 2048 {
            break;
        }
    }
    str_in
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let num_procs = args.flag_num.unwrap_or(4);

    let bin = args.arg_executable.clone();

    let mut rank_map = HashMap::new();

    for i in 0..num_procs {
        let child = Command::new(&bin)
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn()
                            .expect("Failed to spawn process!");

        rank_map.insert(child.id(), i);
    }

    let listener = TcpListener::bind("127.0.0.1:31337").unwrap();
    let mut mailbox = Mailbox::new();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let json = read_from_stream(&mut stream);
                let mut req: CommRequest<String> = json::decode(&json).expect("Invalid json");
                if let CommRequestType::Control(ref ctrl) = req.req_type() {
                    match *ctrl {
                        ControlTy::GetMyRank => {
                            let to_send = format!("{}", rank_map[&req.pid()]);
                            stream.write(to_send.as_bytes());
                        }
                        ControlTy::NumProcs => {
                            let to_send = format!("{}", rank_map.keys().len());
                            stream.write(to_send.as_bytes());
                        }
                        _ => panic!("Invalid control request from process"),
                    }
                    continue;
                }

                if req.is_send() {
                    let pid = req.pid();
                    req.set_src(RequestProc::Process(rank_map[&pid]))
                } else {
                    let pid = req.pid();
                    req.set_dest(RequestProc::Process(rank_map[&pid]))
                }

                if let Some((ref mail, ref mut stream_r)) = mailbox.pop_matching_mail(&req) {
                    match req.is_send() {
                        true => {
                            stream_r.write(&json::encode(&req)
                                                .expect("json encode failed!")
                                                .as_bytes());
                            let ack =
                                CommRequest::<u32>::new(None,
                                                        None,
                                                        u64::max_value(),
                                                        None,
                                                        CommRequestType::Control(ControlTy::Ack),
                                                        0);
                            stream.write(&json::encode(&ack)
                                              .expect("json encode failed!")
                                              .as_bytes());
                        }
                        false => {
                            stream.write(mail.req.as_bytes());
                            let ack =
                                CommRequest::<u32>::new(None,
                                                        None,
                                                        u64::max_value(),
                                                        None,
                                                        CommRequestType::Control(ControlTy::Ack),
                                                        0);
                            stream_r.write(&json::encode(&ack)
                                                .expect("json encode failed!")
                                                .as_bytes());
                        }
                    }
                } else {
                    mailbox.insert_mail(&req, &stream);
                }
            }
            Err(e) => {}
        }
    }
}
