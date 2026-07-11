// ============================================================
// Módulo de Pattern Scanning
// Busca firmas de bytes en la memoria del proceso
// ============================================================

/// Resultado de un escaneo de patrón
#[derive(Debug, Clone)]
#[allow(dead_code)] // `offset` forma parte de la API pública del ScanResult
pub struct ScanResult {
    /// Offset relativo al inicio del módulo
    pub offset: usize,
    /// Dirección absoluta (base + offset)
    pub address: usize,
}

/// Escanea un patrón de bytes con wildcards
/// Formato del patrón: "48 89 4C 24 ?? 48 83 EC"
/// ?? = wildcard (cualquier byte)
pub fn pattern_scan(memory: &[u8], pattern: &str, base_address: usize) -> Vec<ScanResult> {
    let parsed = parse_pattern(pattern);
    if parsed.is_empty() {
        return Vec::new();
    }

    let mut results = Vec::new();
    let pat_len = parsed.len();

    if memory.len() < pat_len {
        return results;
    }

    for i in 0..=(memory.len() - pat_len) {
        let mut found = true;
        for (j, &pat_byte) in parsed.iter().enumerate() {
            if let Some(expected) = pat_byte {
                if memory[i + j] != expected {
                    found = false;
                    break;
                }
            }
            // None = wildcard, siempre coincide
        }
        if found {
            results.push(ScanResult {
                offset: i,
                address: base_address + i,
            });
        }
    }

    results
}

/// Escanea un patrón y devuelve solo el primer resultado
#[allow(dead_code)] // helper de conveniencia de la API de scanning
pub fn pattern_scan_first(memory: &[u8], pattern: &str, base_address: usize) -> Option<ScanResult> {
    let parsed = parse_pattern(pattern);
    if parsed.is_empty() {
        return None;
    }

    let pat_len = parsed.len();
    if memory.len() < pat_len {
        return None;
    }

    for i in 0..=(memory.len() - pat_len) {
        let mut found = true;
        for (j, &pat_byte) in parsed.iter().enumerate() {
            if let Some(expected) = pat_byte {
                if memory[i + j] != expected {
                    found = false;
                    break;
                }
            }
        }
        if found {
            return Some(ScanResult {
                offset: i,
                address: base_address + i,
            });
        }
    }

    None
}

/// Busca strings ASCII/UTF-8 en la memoria
pub fn scan_string(memory: &[u8], target: &str, base_address: usize) -> Vec<ScanResult> {
    let target_bytes = target.as_bytes();
    let mut results = Vec::new();

    if memory.len() < target_bytes.len() {
        return results;
    }

    for i in 0..=(memory.len() - target_bytes.len()) {
        if &memory[i..i + target_bytes.len()] == target_bytes {
            // Verificar que es un string completo y no un substring de otro token.
            // Ambos límites deben ser un separador válido (inicio/fin de buffer,
            // null terminator, o carácter no alfanumérico) para descartar
            // coincidencias como "Item" dentro de "ItemSelectable".
            let before_ok = i == 0 || memory[i - 1] == 0 || !memory[i - 1].is_ascii_alphanumeric();
            let after_pos = i + target_bytes.len();
            let after_ok = after_pos >= memory.len() || memory[after_pos] == 0 || !memory[after_pos].is_ascii_alphanumeric();

            if before_ok && after_ok {
                results.push(ScanResult {
                    offset: i,
                    address: base_address + i,
                });
            }
        }
    }

    results
}

/// Busca todas las strings legibles en un rango de memoria
#[allow(dead_code)] // utilidad para volcados exploratorios de strings
pub fn scan_all_strings(memory: &[u8], base_address: usize, min_length: usize) -> Vec<(usize, String)> {
    let mut results = Vec::new();
    let mut current_string = Vec::new();
    let mut start_offset = 0;

    for (i, &byte) in memory.iter().enumerate() {
        if byte.is_ascii_graphic() || byte == b' ' {
            if current_string.is_empty() {
                start_offset = i;
            }
            current_string.push(byte);
        } else {
            if current_string.len() >= min_length {
                if let Ok(s) = String::from_utf8(current_string.clone()) {
                    results.push((base_address + start_offset, s));
                }
            }
            current_string.clear();
        }
    }

    // Último string
    if current_string.len() >= min_length {
        if let Ok(s) = String::from_utf8(current_string) {
            results.push((base_address + start_offset, s));
        }
    }

    results
}

/// Busca referencias a una dirección (LEA, MOV con RIP-relative)
#[allow(dead_code)] // usado para resolución manual de XRefs
pub fn scan_references(memory: &[u8], target_address: usize, base_address: usize) -> Vec<ScanResult> {
    let mut results = Vec::new();

    if memory.len() < 7 {
        return results;
    }

    for i in 0..memory.len() - 7 {
        // LEA reg, [rip + offset] -> bytes: 48 8D 05/0D/15/1D/25/2D/35/3D xx xx xx xx
        // También: 4C 8D 05/0D/...
        let has_rex = memory[i] == 0x48 || memory[i] == 0x4C;
        let is_lea = memory[i + 1] == 0x8D;

        if has_rex && is_lea && i + 7 <= memory.len() {
            let modrm = memory[i + 2];
            // mod=00, rm=101 (RIP-relative) -> modrm & 0xC7 == 0x05
            if modrm & 0xC7 == 0x05 {
                let rel_offset = i32::from_le_bytes([
                    memory[i + 3],
                    memory[i + 4],
                    memory[i + 5],
                    memory[i + 6],
                ]);
                let resolved = (base_address + i + 7).wrapping_add(rel_offset as usize);

                if resolved == target_address {
                    results.push(ScanResult {
                        offset: i,
                        address: base_address + i,
                    });
                }
            }
        }
    }

    results
}

/// Si en `offset` hay una instrucción `LEA reg, [rip+disp]`
/// (REX.W/REX.R + 0x8D + ModR/M con mod=00,rm=101), devuelve la dirección
/// absoluta que carga. Devuelve `None` si los bytes no son ese LEA.
/// Centraliza el reconocimiento del patrón usado en todo el dumper.
pub fn try_resolve_lea_at(memory: &[u8], offset: usize, base_address: usize) -> Option<usize> {
    if offset + 7 > memory.len() {
        return None;
    }
    let has_rex = memory[offset] == 0x48 || memory[offset] == 0x4C;
    let is_lea = memory[offset + 1] == 0x8D;
    if !(has_rex && is_lea) {
        return None;
    }
    // mod=00, rm=101 (RIP-relative) -> modrm & 0xC7 == 0x05
    if memory[offset + 2] & 0xC7 != 0x05 {
        return None;
    }
    resolve_rip_relative(memory, offset, base_address, 7)
}

/// Resuelve una dirección RIP-relative desde un offset en memoria
/// Asume instrucción de 7 bytes (REX + opcode + modrm + 4 bytes imm)
pub fn resolve_rip_relative(memory: &[u8], instruction_offset: usize, base_address: usize, instr_len: usize) -> Option<usize> {
    if instruction_offset + instr_len > memory.len() {
        return None;
    }

    // El displacement de 4 bytes empieza en instruction_offset + (instr_len - 4)
    let disp_offset = instruction_offset + instr_len - 4;
    if disp_offset + 4 > memory.len() {
        return None;
    }

    let rel_offset = i32::from_le_bytes([
        memory[disp_offset],
        memory[disp_offset + 1],
        memory[disp_offset + 2],
        memory[disp_offset + 3],
    ]);

    let next_ip = base_address + instruction_offset + instr_len;
    Some(next_ip.wrapping_add(rel_offset as usize))
}

/// Parsea un patrón de string a bytes opcionales
fn parse_pattern(pattern: &str) -> Vec<Option<u8>> {
    pattern
        .split_whitespace()
        .map(|byte_str| {
            if byte_str == "??" || byte_str == "?" {
                None
            } else {
                u8::from_str_radix(byte_str, 16).ok()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_scan() {
        let memory = vec![0x48, 0x89, 0x4C, 0x24, 0x08, 0x48, 0x83, 0xEC];
        let results = pattern_scan(&memory, "48 89 4C 24 ?? 48", 0x140000000);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].offset, 0);
    }

    #[test]
    fn test_pattern_scan_wildcard() {
        let memory = vec![0x48, 0x89, 0x4C, 0x24, 0x08, 0x48, 0x89, 0x4C, 0x24, 0xFF];
        let results = pattern_scan(&memory, "48 89 4C 24 ??", 0);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_string_scan() {
        let mut memory = vec![0u8; 100];
        let test_str = b"Player location";
        memory[10..10 + test_str.len()].copy_from_slice(test_str);
        memory[10 + test_str.len()] = 0; // null terminator
        let results = scan_string(&memory, "Player location", 0x140000000);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_string_scan_rejects_substring() {
        // "Item" no debe coincidir dentro de "ItemSelectable"
        let memory = b"\0ItemSelectable\0".to_vec();
        assert!(scan_string(&memory, "Item", 0).is_empty());
        assert_eq!(scan_string(&memory, "ItemSelectable", 0).len(), 1);
        // pero sí como token independiente
        let standalone = b"\0Item\0".to_vec();
        assert_eq!(scan_string(&standalone, "Item", 0).len(), 1);
    }

    #[test]
    fn test_try_resolve_lea_at() {
        let base = 0x140000000usize;
        // LEA rax, [rip+0x10]: 48 8D 05 10 00 00 00 -> base + 7 + 0x10
        let mem = vec![0x48, 0x8D, 0x05, 0x10, 0x00, 0x00, 0x00];
        assert_eq!(try_resolve_lea_at(&mem, 0, base), Some(base + 7 + 0x10));
        // REX.R + reg r9: 4C 8D 0D ...
        let mem_r = vec![0x4C, 0x8D, 0x0D, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(try_resolve_lea_at(&mem_r, 0, base), Some(base + 7));
        // MOV, no LEA -> None
        let mem_mov = vec![0x48, 0x89, 0x05, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(try_resolve_lea_at(&mem_mov, 0, base), None);
        // LEA con desplazamiento por registro (modrm != RIP) -> None
        let mem_reg = vec![0x48, 0x8D, 0x40, 0x10, 0x00, 0x00, 0x00];
        assert_eq!(try_resolve_lea_at(&mem_reg, 0, base), None);
        // desplazamiento negativo (-7) -> base
        let mem_neg = vec![0x48, 0x8D, 0x05, 0xF9, 0xFF, 0xFF, 0xFF];
        assert_eq!(try_resolve_lea_at(&mem_neg, 0, base), Some(base));
    }
}
