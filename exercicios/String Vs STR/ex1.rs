/*
Crie uma variável do tipo &str contendo seu nome completo e imprima:

Nome: <seu nome>
Quantidade de caracteres: <tamanho>


🔹 Regras:

Use &str (let nome: &str = "...";)

Não pode converter para String

Use .len() para contar os bytes (não caracteres!)
*/

fn main() {
    let nome: &str = "Gabriel Almeida Costa";

    println!("Nome: {nome}");
    println!("Quantidade de caracteres (bytes): {}", nome.len());
}
