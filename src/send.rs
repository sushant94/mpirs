use rustc_serialize::json;
use libc;
use mpi_comm::MPIComm;
use comm_request::CommRequest;
use comm_request::CommRequestType;
use comm_request::RequestProc;
use comm_request::MType;
use std::sync::mpsc::{Receiver};
use std::sync::mpsc::channel;
use std::thread;
use receiver_traits::Message;
use std::fmt::Debug;
use rustc_serialize::Encodable;
use rustc_serialize::Decodable;
use std::io::prelude::*;
use std::net::TcpStream;
use utils;


// Functions in the Send module
pub fn mpi_isend<T>(buf: &T,
                    dest: RequestProc,
                    tag: u64,
                    comm: MPIComm)
                    -> Receiver<CommRequest<T>>
    where T: 'static + Debug + Clone + Encodable + Decodable + Send
{
    let pid = utils::pid();
    let commreq = CommRequest::<T>::new(None,
                                       Some(dest),
                                       tag,
                                       Some(buf.clone()),
                                       CommRequestType::Message(MType::MSend),
                                       pid);
    let commreq_json = json::encode(&commreq).unwrap();
    // create channel
    let (tx, rx) = channel::<CommRequest<T>>();
    // spawn thread
    thread::spawn(move || {
        // in thread tcpstream connect, write and read
        let mut stream = TcpStream::connect("127.0.0.1:31337").unwrap();
        let _ = stream.write(&commreq_json.as_bytes());
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

        if !str_in.is_empty() {
            tx.send(json::decode(&str_in).expect("Invalid json"));
        }

    });

    // return receiver
    rx
}

pub fn mpi_send<T>(buf: &T,
                   dest: RequestProc,
                   tag: u64,
                   comm: MPIComm)
    where T: 'static + Debug + Clone + Encodable + Decodable + Send
{
    let rx: Receiver<CommRequest<T>> = mpi_isend(buf, dest, tag, comm);
    rx.wait();
}
