#[no_mangle]
pub extern "C" fn rename_path() -> i32 {
    let old_path = "example.txt";
    let new_path = "renamed_example.txt";

    match std::fs::rename(old_path, new_path) {
        Ok(_) => 0,  // Sucesso
        Err(_) => -1, // Erro
    }
}

