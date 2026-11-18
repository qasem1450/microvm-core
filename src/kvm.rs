use std::fs::OpenOptions;
use std::os::unix::io::{AsRawFd, RawFd};

pub struct KvmSystem {
    pub fd: RawFd,
}

impl KvmSystem {
    pub fn open() -> Result<Self, String> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/kvm")
            .map_err(|e| format!("Cannot open /dev/kvm: {}", e))?;

        Ok(Self {
            fd: file.as_raw_fd(),
        })
    }
}
