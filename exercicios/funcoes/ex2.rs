/*
 Exercício 2 — Função para Verificar se o Número é Par

Crie uma função chamada eh_primo que recebe um inteiro u32 e retorna bool.

Regras:

Deve retornar true se for par, false caso contrário.

No main, teste a função chamando-a com os valores 7, 10 e 13.
*/
fn eh_par(n: u32) -> bool {
    n % 2 == 0
}
fn main() {
    let numero = 7;
    println!("O número {} é par? {}", numero, eh_par(numero));
}
// substitua o numero para testar outros valores