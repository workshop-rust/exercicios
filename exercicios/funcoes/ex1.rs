/*
Exercício 3 — Função para Calcular soma de Três Números

Crie uma função chamada soma3 que recebe três números f64 e retorna a média deles (f64).

Regras:


Imprima a frase:
"A soma é: X" (substitua X pelo valor retornado).
*/
fn soma(a: i32, b: i32, c: i32) -> i64 {
    a + b + c;
}
//ache o erro no código acima e corrija-o
fn main() {
    let resultado = soma(10, 20, 30);
    println!("A soma dos números é: {}", resultado);
}

// dica: tipo de dado e retorno