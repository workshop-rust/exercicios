// intervalo de valores do tipo i8: -128 a 127
fn main() {
    let mut inteiro_com_sinal: i8 = -10;

    println!("Valor inicial do inteiro com sinal (i8): {}", inteiro_com_sinal);

    inteiro_com_sinal = 200; // erro: valor fora do intervalo de i8 (-128 a 127)

    println!("Valor alterado do inteiro com sinal (i8): {}", inteiro_com_sinal);

    // Como corrigir o erro acima?
}
