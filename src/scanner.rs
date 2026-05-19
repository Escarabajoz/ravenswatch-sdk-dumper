// AOB scan engine with wildcard support and RIP-relative resolution

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub offset: usize,
    pub address: usize,
}

/// Scans a byte pattern with wildcards.
/// Pattern format: "48 89 4C 24 ?? 48 83 EC"
/// ?? = wildcard (any byte)
pub fn pattern_scan(memory: &[u8], pattern: &str, base_address: usize) -> Vec<ScanResult> {
    let parsed = parse_pattern(pattern);
    let mut results = Vec::new();
    if memory.len() < parsed.len() { return results; }

    'outer: for i in 0..memory.len() - parsed.len() {
        for (j, expected) in parsed.iter().enumerate() {
            if let Some(byte) = expected {
                if memory[i + j] != *byte { continue 'outer; }
            }
        }
        results.push(ScanResult { offset: i, address: base_address + i });
    }
    results
}

pub fn pattern_scan_first(memory: &[u8], pattern: &str, base: usize) -> Option<ScanResult> {
    pattern_scan(memory, pattern, base).into_iter().next()
}

/// Resolves a RIP-relative LEA/MOV destination address.
pub fn resolve_rip_relative(memory: &[u8], offset: usize, base: usize, instr_len: usize) -> Option<usize> {
    if offset + instr_len > memory.len() { return None; }
    let imm_off = offset + instr_len - 4;
    let rel = i32::from_le_bytes([
        memory[imm_off], memory[imm_off+1],
        memory[imm_off+2], memory[imm_off+3],
    ]);
    Some((base + offset + instr_len).wrapping_add(rel as usize))
}

fn parse_pattern(pattern: &str) -> Vec<Option<u8>> {
    pattern.split_whitespace().map(|tok| {
        if tok == "??" || tok == "?" { None }
        else { u8::from_str_radix(tok, 16).ok() }
    }).collect()
}
