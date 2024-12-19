use std::collections::HashSet;

#[tauri::command]
fn generate_function_signature(code: &str) -> String {
    let opcode_skip: HashSet<&str> = [
        "jmp", "je", "jne", "jle", "jl", "jge", "jg", "call"
    ].into_iter().collect();

    let lines: Vec<&str> = code.lines().collect();
    let mut signature = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let (bytes_part, instruction_part) = match line.split_once('-') {
            Some((bytes, instr)) => (bytes.trim(), instr.trim()),
            None => continue,
        };

        let first_word = instruction_part
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_matches('"');

        let should_skip = opcode_skip.iter()
            .any(|&skip_op| first_word == skip_op);

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

        if should_skip && !bytes.is_empty() {
            signature.push(bytes[0].clone());
            for _ in 1..bytes.len() {
                signature.push("??".to_string());
            }
        } else {
            for (i, byte) in bytes.into_iter().enumerate() {
                if i < 2 {
                    signature.push(byte);
                } else {
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
