//! Module that defines and handles communications
//!
//! `CommRequest` is the final structure that encloses the data to be sent within a struct that
//! contains meta-data required for effective communication.

use rustc_serialize::{Encodable, Decodable, json};
use std::hash;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Differentiate between communications and simlpe requests
#[derive(Debug, Copy, Clone, RustcEncodable, RustcDecodable)]
pub enum CommRequestType {
    /// Message from one process to another
    Message(MType),
    /// Message querying mpirun
    Control(ControlTy),
}

#[derive(Debug, Copy, Clone, RustcEncodable, RustcDecodable)]
pub enum MType {
    MSend,
    MRecv,
}

/// Information requested from mpirun
#[derive(Debug, Copy, Clone, RustcEncodable, RustcDecodable)]
pub enum ControlTy {
    /// Get rank of the process in the communicator
    GetMyRank,
    /// Get number of processes
    NumProcs,
    /// Acknowledge a send when recv is successful
    Ack,
    Exit,
    Nop,
}

#[derive(Debug, Copy, Clone, RustcEncodable, RustcDecodable, PartialEq, Eq, Hash)]
pub enum RequestProc {
    /// Basic point-to-point message send / recv
    Process(usize),
    /// No specific process
    Any,
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct CommRequest<T: Debug + Clone + Encodable + Decodable> {
    /// Filled in by mpirun automatically. Not set by the sending process
    src: Option<RequestProc>,
    dest: Option<RequestProc>,
    /// Message Tag
    tag: u64,
    pty: PhantomData<T>,
    /// Actual data to be sent
    data: Option<String>,
    /// Type of request
    req_ty: CommRequestType,
    pid: u32,
}

impl<T: Debug + Clone + Encodable + Decodable> CommRequest<T> {
    pub fn new(src: Option<RequestProc>,
               dest: Option<RequestProc>,
               tag: u64,
               data: Option<T>,
               ty: CommRequestType,
               pid: u32)
               -> CommRequest<T> {
        let json_data = if data.is_some() {
            json::encode(data.as_ref().unwrap()).ok()
        } else {
            None
        };

        CommRequest {
            src: src,
            dest: dest,
            tag: tag,
            data: json_data,
            pty: PhantomData,
            req_ty: ty,
            pid: pid,
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

    pub fn data(&self) -> Option<String> {
        self.data.clone()
    }

    pub fn req_type(&self) -> CommRequestType {
        self.req_ty
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn set_src(&mut self, src: RequestProc) {
        self.src = Some(src)
    }

    pub fn set_dest(&mut self, dst: RequestProc) {
        self.dest = Some(dst)
    }

    pub fn is_send(&self) -> bool {
        if let CommRequestType::Message(MType::MSend) = self.req_ty {
            true
        } else {
            false
        }
    }

    pub fn is_recv(&self) -> bool {
        if let CommRequestType::Message(MType::MRecv) = self.req_ty {
            true
        } else {
            false
        }
    }

    pub fn is_src_any(&self) -> bool {
        if let Some(RequestProc::Any) = self.src {
            true
        } else {
            false
        }
    }

    pub fn is_dst_any(&self) -> bool {
        if let Some(RequestProc::Any) = self.dest {
            true
        } else {
            false
        }
    }

    pub fn is_request_control(&self) -> bool {
        if let CommRequestType::Control(_) = self.req_ty {
            true
        } else {
            false
        }
    }

    pub fn is_request_message(&self) -> bool {
        !self.is_request_control()
    }
}

pub trait Extract {
    type DType: Clone + Debug;
    fn data(&self) -> Option<Self::DType>;
}

impl<T: Clone + Debug + Encodable + Decodable> Extract for CommRequest<T> {
    type DType = T;
    fn data(&self) -> Option<Self::DType> {
        let x: Option<T> = json::decode(self.data.as_ref().unwrap()).ok();
        x
    }
}
