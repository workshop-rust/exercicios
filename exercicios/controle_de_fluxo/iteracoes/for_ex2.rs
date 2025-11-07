/*
📝 Exercício 2 — Contagem regressiva usando range invertido

Usando um for, faça uma contagem regressiva de 10 até 1 usando um range invertido (com .rev()).
Depois da contagem, exiba "Feliz Ano Novo!".
*/

fn main(){
    for x in (1..=10).rev(){
        println!("{x}");
    }
    println!("Feliz ano novo!");
}