//! Module that defines and handles communications
//!
//! `CommRequest` is the final structure that encloses the data to be sent within a struct that
//! contains meta-data required for effective communication.

use rustc_serialize::Encodable;
use std::hash;
use std::fmt::Debug;

/// Differentiate between communications and simlpe requests
#[derive(Debug, Copy, Clone, RustcEncodable, RustcDecodable)]
pub enum CommRequestType {
    /// Message from one process to another
    Message,
    /// Message querying mpirun
    Control(ControlTy),
}

/// Information requested from mpirun
#[derive(Debug, Copy, Clone, RustcEncodable, RustcDecodable)]
pub enum ControlTy {
    /// Get rank of the process in the communicator
    GetMyRank,
}

#[derive(Debug, Copy, Clone, RustcEncodable, RustcDecodable, PartialEq, Eq, Hash)]
pub enum RequestProc {
    /// Basic point-to-point message send / recv
    Process(usize),
    /// No specific process
    Any,
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct CommRequest<T: Debug + Clone + Encodable> {
    /// Filled in by mpirun automatically. Not set by the sending process
    src: Option<RequestProc>,
    dest: Option<RequestProc>,
    /// Message Tag
    tag: u64,
    /// Actual data to be sent
    data: T,
    /// Type of request
    req_ty: CommRequestType,
}

impl<T: Debug + Clone + Encodable> CommRequest<T> {
    pub fn new(src: Option<RequestProc>,
               dest: Option<RequestProc>,
               tag: u64,
               data: T,
               ty: CommRequestType)
               -> CommRequest<T> {
        CommRequest {
            src: src,
            dest: dest,
            tag: tag,
            data: data,
            req_ty: ty,
        }
    }

    pub fn src(&self) -> Option<RequestProc> {
        self.src
    }

    pub fn dst(&self) -> Option<RequestProc> {
        self.dest
    }

    pub fn tag(&self) -> u64 {
        self.tag
    }

    pub fn data(&self) -> T {
        self.data.clone()
    }

    pub fn set_source(&mut self, src: RequestProc) {
        self.src = Some(src)
    }

    pub fn is_send(&self) -> bool {
        unimplemented!()
    }

    pub fn is_recv(&self) -> bool {
        unimplemented!()
    }

    pub fn is_src_any(&self) -> bool {
        unimplemented!()
    }

    pub fn is_dst_any(&self) -> bool {
        unimplemented!()
    }

    pub fn is_request_control(&self) -> bool {
        unimplemented!()
    }

    pub fn is_request_message(&self) -> bool {
        !self.is_request_control()
    }
}

pub trait Extract {
    type DType: Clone + Debug;
    fn data(&self) -> Self::DType;
}

impl<T: Clone + Debug + Encodable> Extract for CommRequest<T> {
    type DType = T;
    fn data(&self) -> Self::DType {
        self.data.clone()
    }
}
