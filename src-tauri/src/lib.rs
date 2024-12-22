#[tauri::command]
fn generate_function_signature(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut signature = Vec::new();
    
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let (bytes_part, _instruction_part) = match line.split_once('-') {
            Some((bytes, instr)) => (bytes.trim(), instr.trim()),
            None => continue,
        };

        // Parse bytes from the line
        let bytes: Vec<String> = bytes_part
            .split_whitespace()
            .flat_map(|b| {
                if b.len() > 2 {
                    b.chars()
                        .collect::<Vec<char>>()
                        .chunks(2)
                        .map(|chunk| chunk.iter().collect::<String>())
                        .collect()
                } else {
                    vec![b.to_string()]
                }
            })
            .collect();

        if bytes.len() == 1 {
            // Single byte instructions (like push rbp) - keep as is
            signature.push(bytes[0].clone());
        } else if bytes.len() >= 2 {
            // Keep the first byte always
            signature.push(bytes[0].clone());
            
            // For REX prefixes (0x48), keep the next byte too
            if bytes[0] == "48" && bytes.len() > 1 {
                signature.push(bytes[1].clone());
                // Wildcard the rest
                for _ in 2..bytes.len() {
                    signature.push("??".to_string());
                }
            } else {
                // For other instructions, wildcard after first byte
                for _ in 1..bytes.len() {
                    signature.push("??".to_string());
                }
            }
        }
    }

    signature.join(" ")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![generate_function_signature])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
