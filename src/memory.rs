use std::ptr;

pub struct GuestMemory {
    pub host_addr: *mut u8,
    pub size: usize,
}

impl GuestMemory {
    pub fn new(size: usize) -> Result<Self, String> {
        let addr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            )
        };

        if addr == libc::MAP_FAILED {
            return Err("Failed to allocate guest memory via mmap".to_string());
        }

        Ok(Self {
            host_addr: addr as *mut u8,
            size,
        })
    }
}

impl Drop for GuestMemory {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.host_addr as *mut libc::c_void, self.size);
        }
    }
}
