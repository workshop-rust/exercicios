/*
📝 Exercício 1 — Contagem regressiva simples

Escreva um programa que usa um while para imprimir os números de 5 até 1, e depois imprima "Fim!".
A saída deve ser parecida com:

5
4
3
2
1
Fim!
*/

fn main(){
    
    let mut contagem = 5;
    while contagem != 0{
    
        println!("{contagem}");
        contagem = contagem-1;
        
    }
    println!("fim");
}