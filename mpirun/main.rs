//! Implements mpirun equivalent

extern crate docopt;
extern crate rustc_serialize;
extern crate mpirs;

use std::thread;
use std::process::{Command, Stdio};
use std::io::Read;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::os::unix::io::RawFd;
use std::os::unix::io::IntoRawFd;
use std::os::unix::io::FromRawFd;
use std::fs::File;
use std::io::Write;

use rustc_serialize::json;

use docopt::Docopt;

use mpirs::comm_request::CommRequest;

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

fn spawn_process(send_channel: Sender<CommRequest>, send_fd: Sender<RawFd>, bin: String, rank: usize) {
    let mut child = Command::new(bin)
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("Failed to spawn process!");

    send_fd.send(child.stdin.unwrap().into_raw_fd()).expect("Failed to forward fd");

    loop {
        let mut bytes_read = [0; 2048];
        let mut str_in = String::new();
        if let Some(stdout) = child.stdout.as_mut() {
            loop {
                let n = stdout.read(&mut bytes_read).expect("Read Error:");
                str_in = format!("{}{}", str_in,
                                String::from_utf8(bytes_read[0..n].to_vec()).unwrap());
                if n < 2048 {
                    break;
                }
            }
        }
        // Work with the string in str_in.
        if !str_in.is_empty() {
            let mut req: CommRequest = json::decode(&str_in).expect("Invalid Json!");
            req.src = Some(rank);
            send_channel.send(req).expect("Send Error:");
        }
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let num_procs = args.flag_num.unwrap_or(4);

    let bin = args.arg_executable.clone();

    // Create n channels to listen on for the master thread
    let (tx, rx) = mpsc::channel();
    let (tx_fd, rx_fd) = mpsc::channel();

    // Code to spawn thread and process here.
    for i in 0..num_procs {
        let tx_ = tx.clone();
        let bin_ = bin.clone();
        let tx_fd_ = tx_fd.clone();
        thread::spawn(move || {
            spawn_process(tx_, tx_fd_, bin_, i);
        });
    }

    let mut proc_stdin = Vec::new();
    for _ in 0..num_procs {
        let fd = rx_fd.recv().expect("fd Recv error");
        unsafe {
            proc_stdin.push(File::from_raw_fd(fd));
        }
    }

    for message in rx.iter() {
        // Perform some action on the buffer.
        let dest = message.dest().unwrap();
        assert!(dest < num_procs);
        let message_json = json::encode(&message).unwrap();
        proc_stdin[dest].write(&message_json.as_bytes()).expect("Write to proc failed:");
    }
}
