//! Comm Request structure

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub enum CommRequestType {
    Message,
    Control,
}

impl Default for CommRequestType {
    fn default() -> CommRequestType {
        CommRequestType::Message
    }
}

#[derive(Debug, Clone, Default, RustcEncodable, RustcDecodable)]
pub struct CommRequest {
    pub src: Option<usize>,
    dest: Option<usize>,
    tag: Option<u64>,
    data: Option<String>,
    req_ty: CommRequestType,
}

impl CommRequest {
    pub fn new(src: Option<usize>,
               dest: Option<usize>,
               tag: Option<u64>,
               data: Option<String>,
               ty: CommRequestType)
               -> CommRequest {
        CommRequest {
            src: src,
            dest: dest,
            tag: tag,
            data: data,
            req_ty: ty,
        }
    }

    pub fn src(&self) -> Option<usize> {
        self.src
    }

    pub fn dest(&self) -> Option<usize> {
        self.dest
    }

    pub fn tag(&self) -> Option<u64> {
        self.tag
    }

    pub fn data(&self) -> Option<String> {
        self.data.clone()
    }
}
