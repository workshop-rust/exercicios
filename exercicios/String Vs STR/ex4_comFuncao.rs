/*
Escreva uma função que recebe uma String, coloca tudo em maiúsculas e retorna a nova string modificada.

Exemplo de uso:

let s = String::from("bom dia");
let s2 = caixa_alta(s);
println!("{s2}"); // BOM DIA


🔹 Regras:

A função deve receber String por valor (ownership)

Retornar a String transformada (-> String)

Use .to_uppercase()
*/
fn main() {
    let s = String::from("bom dia");
    let s2 = caixa_alta(s);
    println!("{s2}"); // BOM DIA
}

fn caixa_alta(mut texto: String) -> String {
    texto = texto.to_uppercase();
    texto
}
