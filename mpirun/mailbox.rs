//! Simple mailbox implementation using `HashMap`s
//!

use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::cmp::Ordering;

use rustc_serialize::Encodable;
use rustc_serialize::json;

use mpirs::comm_request::{CommRequest, RequestProc};

macro_rules! get_value {
    ($m: ident, $k: expr) => {
        match $k {
            KT::H1(ref k) => $m.h1.get(k).clone(),
            KT::H2(ref k) => $m.h2.get(k).clone(),
        }
    }
}

macro_rules! insert_value {
    ($m: ident, $k: expr, $v: expr) => {
        match $k {
            KT::H1(ref k) => $m.h1.insert(k.clone(), $v.clone()),
            KT::H2(ref k) => $m.h2.insert(k.clone(), $v.clone()),
        }
    }
}

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

impl Mail {
    pub fn new<T>(id: usize, port: usize, req: &CommRequest<T>) -> Mail
        where T: Debug + Clone + Encodable
    {
        Mail {
            id: id,
            port: port,
            req: json::encode(req).unwrap(),
        }
    }
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
    h1: HashMap<MailboxKey, VecDeque<Mail>>,
    h2: HashMap<MailboxKey, VecDeque<Mail>>,
    id: usize,
}

impl Mailbox {
    pub fn new() -> Mailbox {
        Mailbox {
            id: 0,
            h1: HashMap::new(),
            h2: HashMap::new(),
        }
    }

    pub fn pop_matching_mail<T>(&mut self, req: &CommRequest<T>) -> Option<Mail>
        where T: Debug + Clone + Encodable
    {
        let keys = Mailbox::mirror_keys(req);
        match keys.len() {
            2 => self.fast_first_union(keys),
            3 => self.fast_first_union_intersect(keys),
            _ => unreachable!(),
        }
    }

    pub fn insert_mail<T>(&mut self, mail: Mail, req: &CommRequest<T>, port: usize)
        where T: Debug + Clone + Encodable
    {
        let mtype = if req.is_send() {
            MessageTy::MSend
        } else {
            MessageTy::MRecv
        };

        let mail = Mail::new(self.id, port, req);
        self.id += 1;

        let h1_key = MailboxKey::new(mtype, req.src().unwrap(), req.tag());
        let h2_key = MailboxKey::new(mtype, req.dst().unwrap(), req.tag());

        if !self.h1.contains_key(&h1_key) {
            self.h1.insert(h1_key, VecDeque::new());
        }

        if let Some(ref mut v) = self.h1.get_mut(&h1_key) {
            v.push_back(mail.clone());
        }
        
        if !self.h2.contains_key(&h2_key) {
            self.h2.insert(h2_key, VecDeque::new());
        }

        if let Some(ref mut v) = self.h2.get_mut(&h2_key) {
            v.push_back(mail.clone());
        }
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
        let (e, v1_, v2_) = { 
            let v1 = get_value!(self, keys[0]);
            let v2 = get_value!(self, keys[1]);

            match (v1, v2) {
                (None, None) => (None, None, None),
                (None, Some(e)) => {
                    let mut e_ = e.clone();
                    let ele = e_.pop_front();
                    (ele, None, Some(e_))
                }
                (Some(e), None) => {
                    let mut e_ = e.clone();
                    let ele = e_.pop_front();
                    (ele, Some(e_), None)
                }
                (Some(e), Some(e_)) => {
                    let mut vec1 = e.clone();
                    let mut vec2 = e_.clone();
                    let ele = match vec1[0].cmp(&vec2[0]) {
                        Ordering::Less => vec1.pop_front(),
                        Ordering::Greater => vec2.pop_front(),
                        Ordering::Equal => unreachable!(),
                    };
                    (ele, Some(vec1), Some(vec2))
                },
            }
        };
        if v1_.is_some() {
            insert_value!(self, keys[0], v1_.unwrap());
        }
        if v2_.is_some() {
            insert_value!(self, keys[1], v2_.unwrap());
        }
        e
    }
}
