use std::path::PathBuf;

pub struct Result {
    pub files: usize,
    pub directories: usize,
    pub less_than_4_k: usize,
    pub between_4_k_8_k: usize,
    pub between_8_k_16_k: usize,
    pub between_16_k_32_k: usize,
    pub between_32_k_64_k: usize,
    pub between_64_k_128_k: usize,
    pub between_128_k_256_k: usize,
    pub between_256_k_512_k: usize,
    pub between_512_k_1_m: usize,
    pub between_1_m_10_m: usize,
    pub between_10_m_100_m: usize,
    pub between_100_m_1_g: usize,
    pub more_than_1_g: usize,
}
pub fn build_result() -> Result {
    Result {
        files: 0,
        directories: 0,

        less_than_4_k: 0,
        between_4_k_8_k: 0,
        between_8_k_16_k: 0,
        between_16_k_32_k: 0,
        between_32_k_64_k: 0,
        between_64_k_128_k: 0,
        between_128_k_256_k: 0,
        between_256_k_512_k: 0,
        between_512_k_1_m: 0,
        between_1_m_10_m: 0,
        between_10_m_100_m: 0,
        between_100_m_1_g: 0,
        more_than_1_g: 0,
    }
}


pub enum ResponseType {
    File,
    Dir,
    DoneDir,
}
pub struct ChanResponse {
    pub t: ResponseType,
    pub path: PathBuf,
    pub len: u64,
}
pub fn build_dir_chan(path: PathBuf) -> ChanResponse {
    ChanResponse {
        t: ResponseType::Dir,
        path,
        len: 0,
    }
}
pub fn build_dir_chan_done() -> ChanResponse {
    ChanResponse {
        t: ResponseType::DoneDir,
        path: PathBuf::new(),
        len: 0,
    }
}
pub fn build_file_chan(size: u64) -> ChanResponse {
    ChanResponse {
        t: ResponseType::File,
        path: PathBuf::new(),
        len: size,
    }
}