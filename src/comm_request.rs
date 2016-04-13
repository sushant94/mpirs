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

#[derive(Debug, Copy, Clone, RustcEncodable, RustcDecodable)]
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
    dest: RequestProc,
    /// Message Tag
    tag: u64,
    /// Actual data to be sent
    data: T,
    /// Type of request
    req_ty: CommRequestType,
}

// Implementing Hash on CommRequest. This allows for faster manipulations and comparisons in mpirs
impl<T: Debug + Clone + Encodable> hash::Hash for CommRequest<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.src.unwrap().hash(state);
        self.dest.hash(state);
        self.tag.hash(state);
    }
}

impl hash::Hash for RequestProc {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        match *self {
            RequestProc::Process(target) => target.hash(state),
            RequestProc::Any => (0xffffffffffffffffu64).hash(state),
        }
    }
}


impl<T: Debug + Clone + Encodable> CommRequest<T> {
    pub fn new(src: Option<RequestProc>,
               dest: RequestProc,
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

    pub fn dest(&self) -> RequestProc {
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
