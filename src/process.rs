// ============================================================
// Módulo de lectura de memoria del proceso
// Abre el proceso del juego y lee su memoria
// ============================================================

use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::ptr;

use winapi::shared::minwindef::{DWORD, FALSE, HMODULE, MAX_PATH};
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::{EnumProcessModulesEx, GetModuleBaseNameW, GetModuleInformation, MODULEINFO};
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use winapi::um::winnt::{HANDLE, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

/// Representa un proceso del juego abierto
pub struct GameProcess {
    pub handle: HANDLE,
    pub pid: u32,
    pub base_address: usize,
    pub module_size: usize,
    pub process_name: String,
}

impl GameProcess {
    /// Abre un proceso por nombre
    pub fn open(name: &str) -> Result<Self, String> {
        let pid = Self::find_pid(name)?;
        let handle = unsafe {
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid)
        };

        if handle.is_null() {
            return Err(format!("No se pudo abrir el proceso (PID: {}). ¿Ejecutas como administrador?", pid));
        }

        let (base_address, module_size) = Self::get_module_info(handle, name)?;

        Ok(GameProcess {
            handle,
            pid,
            base_address,
            module_size,
            process_name: name.to_string(),
        })
    }

    /// Busca el PID de un proceso por nombre
    fn find_pid(name: &str) -> Result<u32, String> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot.is_null() {
                return Err("No se pudo crear snapshot de procesos".to_string());
            }

            let mut entry: PROCESSENTRY32W = mem::zeroed();
            entry.dwSize = mem::size_of::<PROCESSENTRY32W>() as DWORD;

            if Process32FirstW(snapshot, &mut entry) == FALSE {
                CloseHandle(snapshot);
                return Err("No se pudo leer el primer proceso".to_string());
            }

            let target = name.to_lowercase();
            loop {
                let exe_name = OsString::from_wide(
                    &entry.szExeFile[..entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(entry.szExeFile.len())]
                );
                let exe_str = exe_name.to_string_lossy().to_lowercase();

                if exe_str == target || exe_str.contains(&target) {
                    let pid = entry.th32ProcessID;
                    CloseHandle(snapshot);
                    return Ok(pid);
                }

                if Process32NextW(snapshot, &mut entry) == FALSE {
                    break;
                }
            }

            CloseHandle(snapshot);
            Err(format!("Proceso '{}' no encontrado. ¿Está el juego abierto?", name))
        }
    }

    /// Obtiene la dirección base y tamaño del módulo principal
    fn get_module_info(handle: HANDLE, name: &str) -> Result<(usize, usize), String> {
        unsafe {
            let mut modules: [HMODULE; 1024] = [ptr::null_mut(); 1024];
            let mut needed: DWORD = 0;

            // LIST_MODULES_ALL = 0x03
            if EnumProcessModulesEx(
                handle,
                modules.as_mut_ptr(),
                (modules.len() * mem::size_of::<HMODULE>()) as DWORD,
                &mut needed,
                0x03,
            ) == FALSE
            {
                return Err("No se pudieron enumerar los módulos del proceso".to_string());
            }

            let count = needed as usize / mem::size_of::<HMODULE>();
            let target = name.to_lowercase();

            for i in 0..count {
                let mut mod_name: [u16; MAX_PATH] = [0; MAX_PATH];
                GetModuleBaseNameW(handle, modules[i], mod_name.as_mut_ptr(), MAX_PATH as DWORD);
                let mod_str = OsString::from_wide(
                    &mod_name[..mod_name.iter().position(|&c| c == 0).unwrap_or(0)]
                );
                let mod_name_str = mod_str.to_string_lossy().to_lowercase();

                if mod_name_str == target || mod_name_str.contains(&target) {
                    let mut mod_info: MODULEINFO = mem::zeroed();
                    GetModuleInformation(
                        handle,
                        modules[i],
                        &mut mod_info,
                        mem::size_of::<MODULEINFO>() as DWORD,
                    );

                    return Ok((
                        mod_info.lpBaseOfDll as usize,
                        mod_info.SizeOfImage as usize,
                    ));
                }
            }

            // Si no encontramos por nombre, usar el primer módulo
            if count > 0 {
                let mut mod_info: MODULEINFO = mem::zeroed();
                GetModuleInformation(
                    handle,
                    modules[0],
                    &mut mod_info,
                    mem::size_of::<MODULEINFO>() as DWORD,
                );
                return Ok((
                    mod_info.lpBaseOfDll as usize,
                    mod_info.SizeOfImage as usize,
                ));
            }

            Err("No se pudo obtener información del módulo principal".to_string())
        }
    }

    /// Lee toda la memoria del módulo principal
    pub fn read_module_memory(&self) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0u8; self.module_size];
        let mut bytes_read: usize = 0;

        let success = unsafe {
            ReadProcessMemory(
                self.handle,
                self.base_address as *const _,
                buffer.as_mut_ptr() as *mut _,
                self.module_size,
                &mut bytes_read,
            )
        };

        if success == FALSE {
            // Intentar leer por páginas (4KB) si falla la lectura completa
            return self.read_module_memory_paged();
        }

        buffer.truncate(bytes_read);
        Ok(buffer)
    }

    /// Lee memoria por páginas (para páginas protegidas)
    fn read_module_memory_paged(&self) -> Result<Vec<u8>, String> {
        let page_size: usize = 0x1000;
        let mut buffer = vec![0u8; self.module_size];
        let mut total_read: usize = 0;

        for offset in (0..self.module_size).step_by(page_size) {
            let read_size = std::cmp::min(page_size, self.module_size - offset);
            let mut bytes_read: usize = 0;

            unsafe {
                let result = ReadProcessMemory(
                    self.handle,
                    (self.base_address + offset) as *const _,
                    buffer[offset..].as_mut_ptr() as *mut _,
                    read_size,
                    &mut bytes_read,
                );

                if result != FALSE {
                    total_read += bytes_read;
                }
            }
        }

        if total_read == 0 {
            return Err("No se pudo leer ninguna página de memoria".to_string());
        }

        Ok(buffer)
    }

    /// Lee bytes en una dirección específica
    pub fn read_bytes(&self, address: usize, size: usize) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0u8; size];
        let mut bytes_read: usize = 0;

        let success = unsafe {
            ReadProcessMemory(
                self.handle,
                address as *const _,
                buffer.as_mut_ptr() as *mut _,
                size,
                &mut bytes_read,
            )
        };

        if success == FALSE || bytes_read == 0 {
            return Err(format!("No se pudo leer en 0x{:X}", address));
        }

        buffer.truncate(bytes_read);
        Ok(buffer)
    }

    /// Lee un valor u64 en una dirección
    pub fn read_u64(&self, address: usize) -> Result<u64, String> {
        let bytes = self.read_bytes(address, 8)?;
        Ok(u64::from_le_bytes(bytes[..8].try_into().unwrap()))
    }

    /// Lee un valor u32 en una dirección
    pub fn read_u32(&self, address: usize) -> Result<u32, String> {
        let bytes = self.read_bytes(address, 4)?;
        Ok(u32::from_le_bytes(bytes[..4].try_into().unwrap()))
    }

    /// Lee un valor i32 en una dirección
    pub fn read_i32(&self, address: usize) -> Result<i32, String> {
        let bytes = self.read_bytes(address, 4)?;
        Ok(i32::from_le_bytes(bytes[..4].try_into().unwrap()))
    }

    /// Lee un valor f32 en una dirección
    pub fn read_f32(&self, address: usize) -> Result<f32, String> {
        let bytes = self.read_bytes(address, 4)?;
        Ok(f32::from_le_bytes(bytes[..4].try_into().unwrap()))
    }

    /// Lee un string (null-terminated) en una dirección
    pub fn read_string(&self, address: usize, max_len: usize) -> Result<String, String> {
        let bytes = self.read_bytes(address, max_len)?;
        let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        Ok(String::from_utf8_lossy(&bytes[..end]).to_string())
    }
}

impl Drop for GameProcess {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { CloseHandle(self.handle); }
        }
    }
}
