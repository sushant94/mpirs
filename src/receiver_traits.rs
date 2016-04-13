use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::RecvError;

pub trait Message {
		type T;

    // Function to read data directly from Receiver
    fn data(&self) -> Option<Self::T>;
    fn wait(&self) -> Self::T;
}


impl<T> Message for Receiver<T> {
	type T = T;

	fn data(&self) -> Option<Self::T> {
	  self.try_recv().ok()
	}

	fn wait(&self) -> Self::T {
	  let res = self.recv().expect("RecvError");
	  res.data()
	}

}