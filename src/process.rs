// Process opening and memory reading via WinAPI
// See full source in repository (truncated module declarations)

use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::{HANDLE, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

pub struct GameProcess {
    pub pid: u32,
    pub handle: HANDLE,
    pub base_address: usize,
    pub module_size: usize,
    pub module_name: String,
}

impl GameProcess {
    pub fn open(_name: &str) -> Result<Self, String> {
        // Full implementation: enumerate processes via CreateToolhelp32Snapshot,
        // find PID by name, OpenProcess, EnumProcessModules to get base/size.
        unimplemented!("See full source")
    }

    pub fn read_module_memory(&self) -> Result<Vec<u8>, String> {
        // Reads [base, base+size) via ReadProcessMemory in pages.
        unimplemented!("See full source")
    }

    pub fn read_bytes(&self, _address: usize, _size: usize) -> Result<Vec<u8>, String> {
        unimplemented!("See full source")
    }
}

impl Drop for GameProcess {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { CloseHandle(self.handle); }
        }
    }
}
