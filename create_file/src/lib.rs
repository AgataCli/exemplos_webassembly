use std::fs::File;
use std::io::{Write};

// Definindo uma função exportada para o WebAssembly
#[no_mangle]
pub extern "C" fn write() -> i32 {
    // Caminho para o diretório de downloads no Android
    //let download_dir = "/storage/emulated/0/Download/example.txt";
    let download_dir = "example.txt";

    // Tentando criar e escrever no arquivo
    match File::create(download_dir) {
        Ok(mut file) => {
            if let Err(_) = writeln!(file, "Este é um teste de escrita no arquivo!") {
                return -1; // Erro na escrita
            }
            return 0; // Sucesso
        }
        Err(_) => return -1, // Erro na criação do arquivo
    }
}
