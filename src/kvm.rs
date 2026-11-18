use std::os::unix::io::RawFd;

pub struct Vcpu {
    pub id: u32,
    pub fd: RawFd,
}

impl Vcpu {
    pub fn new(kvm_fd: RawFd, id: u32) -> Result<Self, String> {
        // استدعاء نظام لإنشاء vCPU جديد داخل KVM
        let vcpu_fd = unsafe {
            libc::ioctl(kvm_fd, KVM_CREATE_VCPU, id as libc::c_ulong)
        };

        if vcpu_fd < 0 {
            return Err("Failed to create KVM vCPU".to_string());
        }

        Ok(Self { id, fd: vcpu_fd })
    }

    pub fn run(&self) -> Result<(), String> {
        println!("[vCPU {}] Starting execution loop...", self.id);
        // هنا يتم عادةً استدعاء KVM_RUN لتنفيذ تعليمات الضيف (Guest Code)
        Ok(())
    }
}

// تعريف رقم استدعاء النظام الخاص بإنشاء vCPU في لينكس
const KVM_CREATE_VCPU: u64 = 0x4401;
