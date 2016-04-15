//! Simple mailbox implementation using `HashMap`s
//!

use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::cmp::Ordering;
use std::net::TcpStream;

use rustc_serialize::{Encodable, Decodable};
use rustc_serialize::json;

use mpirs::comm_request::{CommRequest, RequestProc};

macro_rules! get_value {
    ($m: ident, $k: expr) => {
        match $k {
            KT::H1(ref k) => $m.h1.get(k).cloned(),
            KT::H2(ref k) => $m.h2.get(k).cloned(),
        }
    }
}

macro_rules! get_mut_value {
    ($m: ident, $k: expr) => {
        match $k {
            KT::H1(ref k) => $m.h1.get_mut(k),
            KT::H2(ref k) => $m.h2.get_mut(k),
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
    // stream: Option<TcpStream>,
    pub req: String,
}

impl Mail {
    pub fn new<T>(id: usize, req: &CommRequest<T>) -> Mail
        where T: Debug + Clone + Encodable + Decodable
    {
        Mail {
            id: id,
            // stream: stream,
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

#[derive(Debug)]
pub struct Mailbox {
    h1: HashMap<MailboxKey, VecDeque<Mail>>,
    h2: HashMap<MailboxKey, VecDeque<Mail>>,
    id: usize,
    stream_map: HashMap<usize, TcpStream>,
}

impl Mailbox {
    pub fn new() -> Mailbox {
        Mailbox {
            id: 0,
            h1: HashMap::new(),
            h2: HashMap::new(),
            stream_map: HashMap::new(),
        }
    }

    pub fn pop_matching_mail<T>(&mut self, req: &CommRequest<T>) -> Option<(Mail, TcpStream)>
        where T: Debug + Clone + Encodable + Decodable
    {
        let keys = Mailbox::mirror_keys(req);
        match keys.len() {
            2 => {
                let mail = self.fast_first_union(keys);
                if let Some(mail_) = mail {
                    let tcp_stream = self.stream_map.remove(&mail_.id).unwrap();
                    Some((mail_, tcp_stream))
                } else {
                    None
                }
            }
            3 => {
                let mail = self.fast_first_union_intersect(keys);
                if let Some(mail_) = mail {
                    let tcp_stream = self.stream_map.remove(&mail_.id).unwrap();
                    Some((mail_, tcp_stream))
                } else {
                    None
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn insert_mail<T>(&mut self, req: &CommRequest<T>, stream: &TcpStream)
        where T: Debug + Clone + Encodable + Decodable
    {
        let mtype = if req.is_send() {
            MessageTy::MSend
        } else {
            MessageTy::MRecv
        };

        let mail = Mail::new(self.id, req);
        let cloned_stream = stream.try_clone().expect("Unable to clone TcpStream");
        self.stream_map.insert(self.id, cloned_stream);
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
        where T: Debug + Clone + Encodable + Decodable
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
        let v1 = get_value!(self, keys[0]).unwrap_or(VecDeque::new());
        let v2 = get_value!(self, keys[1]).unwrap_or(VecDeque::new());
        let v3 = get_value!(self, keys[2]).unwrap_or(VecDeque::new());

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = 0;

        let mut found: Option<Mail> = None;

        loop {
            if k >= v3.len() {
                break;
            }
            if i >= v1.len() && j >= v2.len() {
                break;
            }

            if i < v1.len() {
                if v1[i] < v3[k] {
                    i += 1;
                } else if v1[i] == v3[k] {
                    found = get_mut_value!(self, keys[0]).unwrap().remove(i);
                    get_mut_value!(self, keys[2]).unwrap().remove(k);
                    break;
                }
            }

            if j < v2.len() {
                if v2[j] < v3[k] {
                    j += 1;
                } else if v2[j] == v3[k] {
                    found = get_mut_value!(self, keys[1]).unwrap().remove(j);
                    get_mut_value!(self, keys[2]).unwrap().remove(k);
                    break;
                }
            }

            if i < v1.len() && j < v2.len() {
                if v1[i] > v3[k] && v2[j] > v3[k] {
                    k += 1;
                }
            }
        }
        found
    }

    fn fast_first_union(&mut self, keys: Vec<KT>) -> Option<Mail> {
        let v1 = get_value!(self, keys[0]).unwrap_or(VecDeque::new());
        let v2 = get_value!(self, keys[1]).unwrap_or(VecDeque::new());

        let ele = if v1.is_empty() && v2.is_empty() {
            None
        } else if v1.is_empty() {
            get_mut_value!(self, keys[1]).unwrap().pop_front()
        } else if v2.is_empty() {
            get_mut_value!(self, keys[0]).unwrap().pop_front()
        } else {
            match v1[0].cmp(&v2[0]) {
                Ordering::Less => get_mut_value!(self, keys[0]).unwrap().pop_front(),
                Ordering::Greater => get_mut_value!(self, keys[1]).unwrap().pop_front(),
                Ordering::Equal => unreachable!(),
            }
        };
        ele
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mpirs::comm_request::{CommRequest, CommRequestType, MType, RequestProc};

    const COMM_TAG: u64 = 42;

    #[test]
    fn box_insert_get_proc_to_proc() {
        let mut mailbox = Mailbox::new();
        let req = CommRequest::new(Some(RequestProc::Process(0)),
                                   Some(RequestProc::Process(1)),
                                   COMM_TAG,
                                   Some(5u64),
                                   CommRequestType::Message(MType::MSend),
                                   1000u32);

        let req_recv = CommRequest::<u64>::new(Some(RequestProc::Process(0)),
                                               Some(RequestProc::Process(1)),
                                               COMM_TAG,
                                               None,
                                               CommRequestType::Message(MType::MRecv),
                                               1000u32);

        mailbox.insert_mail(&req, &get_tcp_stream());
        assert!(mailbox.pop_matching_mail(&req_recv).is_some());
    }

    #[test]
    fn box_insert_get_proc_to_any() {
        let mut mailbox = Mailbox::new();
        let req = CommRequest::<u64>::new(Some(RequestProc::Process(0)),
                                          Some(RequestProc::Process(1)),
                                          COMM_TAG,
                                          Some(5u64),
                                          CommRequestType::Message(MType::MSend),
                                          1000u32);

        let req_recv = CommRequest::<u64>::new(Some(RequestProc::Any),
                                               Some(RequestProc::Process(1)),
                                               COMM_TAG,
                                               None,
                                               CommRequestType::Message(MType::MRecv),
                                               1000u32);

        mailbox.insert_mail(&req, &get_tcp_stream());
        assert!(mailbox.pop_matching_mail(&req_recv).is_some());
    }

    #[test]
    fn box_insert_get_any_to_proc() {
        let mut mailbox = Mailbox::new();
        let req = CommRequest::<u64>::new(Some(RequestProc::Process(0)),
                                          Some(RequestProc::Any),
                                          COMM_TAG,
                                          Some(5u64),
                                          CommRequestType::Message(MType::MSend),
                                          1000u32);

        let req_recv = CommRequest::<u64>::new(Some(RequestProc::Process(0)),
                                               Some(RequestProc::Process(1)),
                                               COMM_TAG,
                                               None,
                                               CommRequestType::Message(MType::MRecv),
                                               1000u32);

        mailbox.insert_mail(&req, &get_tcp_stream());
        assert!(mailbox.pop_matching_mail(&req_recv).is_some());
    }

    #[test]
    fn box_insert_get_any_to_any() {
        let mut mailbox = Mailbox::new();
        let req = CommRequest::<u64>::new(Some(RequestProc::Process(0)),
                                          Some(RequestProc::Any),
                                          COMM_TAG,
                                          Some(5u64),
                                          CommRequestType::Message(MType::MSend),
                                          1000u32);

        let req_recv = CommRequest::<u64>::new(Some(RequestProc::Any),
                                               Some(RequestProc::Process(1)),
                                               COMM_TAG,
                                               None,
                                               CommRequestType::Message(MType::MRecv),
                                               1000u32);

        mailbox.insert_mail(&req, &get_tcp_stream());
        assert!(mailbox.pop_matching_mail(&req_recv).is_some());
    }

    #[test]
    fn box_proc_2_recv_any() {
        let mut mailbox = Mailbox::new();
        let req = CommRequest::<u64>::new(Some(RequestProc::Process(0)),
                                          Some(RequestProc::Process(2)),
                                          COMM_TAG,
                                          Some(5u64),
                                          CommRequestType::Message(MType::MSend),
                                          1000u32);

        let req_recv = CommRequest::<u64>::new(Some(RequestProc::Any),
                                               Some(RequestProc::Process(1)),
                                               COMM_TAG,
                                               None,
                                               CommRequestType::Message(MType::MRecv),
                                               1000u32);

        mailbox.insert_mail(&req, &get_tcp_stream());
        assert!(mailbox.pop_matching_mail(&req_recv).is_none());
    }

    #[test]
    fn box_inorder() {
        let mut mailbox = Mailbox::new();
        let req = CommRequest::<u64>::new(Some(RequestProc::Process(0)),
                                          Some(RequestProc::Process(1)),
                                          COMM_TAG,
                                          Some(5u64),
                                          CommRequestType::Message(MType::MSend),
                                          1000u32);

        let req_1 = CommRequest::<u64>::new(Some(RequestProc::Process(0)),
                                            Some(RequestProc::Any),
                                            COMM_TAG,
                                            Some(5u64),
                                            CommRequestType::Message(MType::MSend),
                                            1000u32);

        let req_recv = CommRequest::<u64>::new(Some(RequestProc::Any),
                                               Some(RequestProc::Process(1)),
                                               COMM_TAG,
                                               None,
                                               CommRequestType::Message(MType::MRecv),
                                               1000u32);

        mailbox.insert_mail(&req, &get_tcp_stream());
        mailbox.insert_mail(&req_1, &get_tcp_stream());

        let rep = mailbox.pop_matching_mail(&req_recv);
        assert!(rep.is_some());
        assert_eq!(rep.unwrap().0.id, 0);
    }

    #[test]
    fn box_inorder_multiple() {
        let mut mailbox = Mailbox::new();
        let req = CommRequest::<u64>::new(Some(RequestProc::Process(0)),
                                          Some(RequestProc::Process(1)),
                                          COMM_TAG,
                                          Some(5u64),
                                          CommRequestType::Message(MType::MSend),
                                          1000u32);

        let req_1 = CommRequest::<u64>::new(Some(RequestProc::Process(0)),
                                            Some(RequestProc::Any),
                                            COMM_TAG,
                                            Some(5u64),
                                            CommRequestType::Message(MType::MSend),
                                            1000u32);

        let req_recv = CommRequest::<u64>::new(Some(RequestProc::Any),
                                               Some(RequestProc::Process(1)),
                                               COMM_TAG,
                                               None,
                                               CommRequestType::Message(MType::MRecv),
                                               1000u32);

        mailbox.insert_mail(&req, &get_tcp_stream());
        mailbox.insert_mail(&req_1, &get_tcp_stream());

        let rep = mailbox.pop_matching_mail(&req_recv);
        assert!(rep.is_some());
        assert_eq!(rep.unwrap().0.id, 0);

        let rep = mailbox.pop_matching_mail(&req_recv);
        assert!(rep.is_some());
        assert_eq!(rep.unwrap().0.id, 1);

        assert!(mailbox.pop_matching_mail(&req_recv).is_none());
    }

    #[test]
    fn box_recv_before_send() {
        let mut mailbox = Mailbox::new();
        let req = CommRequest::<u64>::new(Some(RequestProc::Process(0)),
                                          Some(RequestProc::Process(1)),
                                          COMM_TAG,
                                          Some(5u64),
                                          CommRequestType::Message(MType::MSend),
                                          1000u32);

        let req_recv = CommRequest::<u64>::new(Some(RequestProc::Any),
                                               Some(RequestProc::Process(1)),
                                               COMM_TAG,
                                               None,
                                               CommRequestType::Message(MType::MRecv),
                                               1000u32);


        mailbox.insert_mail(&req_recv, &get_tcp_stream());
        let rep = mailbox.pop_matching_mail(&req);
        assert!(rep.is_some());
        assert_eq!(rep.unwrap().0.id, 0);
        assert!(mailbox.pop_matching_mail(&req).is_none());
    }
}
