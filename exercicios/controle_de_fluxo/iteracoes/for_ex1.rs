/*
📝 Exercício 1 — Imprimindo um vetor

Crie um vetor chamado numeros contendo os valores [1, 3, 5, 7, 9].
Use um for para iterar sobre esse vetor e imprimir cada número na tela no formato:

Número: 1
Número: 3
...
*/

fn main() {
    let numeros = vec![1, 3, 5, 7, 9];
    for numero in numeros {
        println!("Número: {}", numero);
    }
}