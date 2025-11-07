/*
Crie uma função chamada fatorial que recebe um número inteiro positivo u32 e retorna seu fatorial.

Regras:

Se n = 0, o resultado deve ser 1.

Use recursão ou loop, você escolhe.

No main, peça o cálculo do fatorial de 5 e imprima o resultado.
*/

fn fatorial -> usize{
    fn fatorial(n: u32) -> u32 {
        if n == 0 {
            1
        } else {
            n * fatorial(n - 1)
        }
    }

    fn main() {
        let resultado = fatorial(5);
        println!("O fatorial de 5 é: {}", resultado);
    }
}