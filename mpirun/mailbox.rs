//! Simple mailbox implementation using `HashMap`s
//!

use std::collections::HashMap;
use std::fmt::Debug;
use std::cmp::Ordering;

use rustc_serialize::Encodable;

use mpirs::comm_request::{CommRequest, RequestProc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MailboxKey {
    key_type: MessageTy,
    actor: RequestProc,
    tag: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MessageTy {
    MSend,
    MRecv,
}

impl MailboxKey {
    fn new(kt: MessageTy, p: RequestProc, t: u64) -> MailboxKey {
        MailboxKey {
            key_type: kt,
            actor: p,
            tag: t,
        }
    }
}

#[derive(Clone, Debug)]
enum KT {
    H1(MailboxKey),
    H2(MailboxKey),
}

#[derive(Clone, Debug)]
pub struct Mail {
    id: usize,
    port: usize,
    req: String,
}

impl PartialEq for Mail {
    fn eq(&self, other: &Mail) -> bool {
        self.id == other.id
    }
}

impl Eq for Mail { }

impl PartialOrd for Mail {
    fn partial_cmp(&self, other: &Mail) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Mail {
    fn cmp(&self, other: &Mail) -> Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(Debug, Clone)]
pub struct Mailbox {
    h1: HashMap<MailboxKey, Vec<Mail>>,
    h2: HashMap<MailboxKey, Vec<Mail>>,
}

impl Mailbox {
    pub fn new() -> Mailbox {
        Mailbox {
            h1: HashMap::new(),
            h2: HashMap::new(),
        }
    }

    pub fn pop_matching_mail<T>(&mut self, req: &CommRequest<T>) -> Option<Mail>
        where T: Debug + Clone + Encodable
    {
        unimplemented!()
    }

    pub fn insert_mail<T>(&mut self, mail: Mail, req: &CommRequest<T>)
        where T: Debug + Clone + Encodable
    {
        unimplemented!()
    }

    fn mirror_keys<T>(req: &CommRequest<T>) -> Vec<KT>
        where T: Debug + Clone + Encodable
    {
        if req.is_send() {
            let mut keys = vec![KT::H1(MailboxKey::new(MessageTy::MRecv,
                                                       req.src().unwrap(),
                                                       req.tag())),
                                KT::H1(MailboxKey::new(MessageTy::MRecv,
                                                       RequestProc::Any,
                                                       req.tag()))];
            if !req.is_dst_any() {
                keys.push(KT::H2(MailboxKey::new(MessageTy::MRecv, req.dst().unwrap(), req.tag())));
            }
            keys
        } else if req.is_recv() {
            let mut keys = vec![KT::H2(MailboxKey::new(MessageTy::MSend,
                                                       req.dst().unwrap(),
                                                       req.tag())),
                                KT::H2(MailboxKey::new(MessageTy::MSend,
                                                       RequestProc::Any,
                                                       req.tag()))];
            if !req.is_src_any() {
                keys.push(KT::H1(MailboxKey::new(MessageTy::MSend, req.src().unwrap(), req.tag())));
            }
            keys
        } else {
            panic!("Request is neither a send nor recv");
        }
    }

    fn fast_first_union_intersect(&mut self, keys: Vec<KT>) -> Option<Mail> {
        unimplemented!()
    }

    fn fast_first_union(&mut self, keys: Vec<KT>) -> Option<Mail> {
        unimplemented!()
    }
}
