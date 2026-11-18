use std::os::unix::io::RawFd;

pub struct Vcpu {
    pub fd: RawFd,
}

impl Vcpu {
    pub fn new(fd: RawFd) -> Self {
        Self { fd }
    }

    pub fn run(&self) -> Result<(), String> {
        println!("[vCPU] Ready to execute on fd: {}", self.fd);
        Ok(())
    }
}
