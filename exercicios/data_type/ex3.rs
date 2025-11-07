// intervalo de valores do tipo u8: 0 a 255
fn main() {
    let mut inteiro_sem_sinal: u8 = 10;

    println!("Valor inicial do inteiro sem sinal (u8): {}", inteiro_sem_sinal);

    inteiro_sem_sinal = -5; // erro: valores negativos não são permitidos para u8

    println!("Valor alterado do inteiro sem sinal (u8): {}", inteiro_sem_sinal);

    // Como corrigir o erro acima?
}
