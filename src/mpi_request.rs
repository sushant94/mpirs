// MPIRequest structure 

#[derive(Debug, Clone, Default)]
pub struct MPIRequest {
    src: Option<u64>,
    dest: Option<u64>,
    tag: Option<u64>
}
