mod kvm;
mod memory;
mod vcpu;

use memory::GuestMemory;

fn main() {
    println!("=== Starting microvm-core (x86_64) ===");

    let mem_size = 16 * 1024 * 1024; // 16 MB
    match GuestMemory::new(mem_size) {
        Ok(mem) => println!("[Memory] Allocated {} bytes at {:p}", mem.size, mem.host_addr),
        Err(e) => eprintln!("[Memory Error] {}", e),
    }

    match kvm::KvmSystem::open() {
        Ok(kvm) => println!("[KVM] System initialized (fd: {})", kvm.fd),
        Err(e) => println!("[KVM Notice] {} (Running in abstraction mode)", e),
    }
}
