use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::RecvError;

use comm_request::Extract;

use std::fmt::Debug;

pub trait Message {
    type T: Extract + Clone + Debug;

    // Function to read data directly from Receiver
    fn data(&self) -> Option<<Self::T as Extract>::DType>;
    fn wait(&self) -> Option<<Self::T as Extract>::DType>;
}


impl<T: Extract + Clone + Debug> Message for Receiver<T> {
	type T = T;

    fn data(&self) -> Option<<Self::T as Extract>::DType> {
        if let Some(ref data) = self.try_recv().ok() {
            data.data()
        } else {
            None
        }
    }

    fn wait(&self) -> Option<<Self::T as Extract>::DType> {
        let res = self.recv().expect("RecvError");
        res.data()
    }
}
