// MPIRequest structure 

#[derive(Debug, Clone, Default)]
pub struct MPIRequest {
    src: Option<usize>,
    dest: Option<usize>,
    tag: Option<u64>
}

impl MPIRequest {
    pub fn new(src: usize, dest: usize, tag: u64) -> MPIRequest{
    	MPIRequest{
    		src: Some(src),
    		dest: Some(dest),
    		tag: Some(tag)
    	}
    }
}
