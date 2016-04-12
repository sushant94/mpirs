// Comm Request structure 

#[derive(Debug, Clone, Default, RustcEncodable)]
pub struct CommRequest {
    src: Option<usize>,
    dest: Option<usize>,
    tag: Option<u64>,
    data: Option<String>
}
