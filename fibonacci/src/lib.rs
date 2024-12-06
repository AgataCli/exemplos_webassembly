#[no_mangle]
pub extern "C" fn funcfib(n: i32) -> i32 {
    // Função recursiva para calcular Fibonacci
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        funcfib(n - 1) + funcfib(n - 2)
    }
}

#[no_mangle]
pub extern "C" fn fib(n: i32) -> i32 {
    // Chama funcfib para calcular Fibonacci de n e armazena o resultado
    let result = funcfib(n);
    // Imprime o resultado na saída padrão
    println!("Fibonacci de {}: {}", n, result);
    // Retorna o resultado para o chamador
    result
}
