#[no_mangle]
pub extern "C" fn yield_execution() -> i32 {
    std::thread::yield_now();
    0 // Sempre retorna sucesso
}

