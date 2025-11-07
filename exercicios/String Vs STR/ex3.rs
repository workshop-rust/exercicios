/*Crie um String com o texto "Rust" e depois:

Adicione " é muito " usando push_str

Adicione 'f', 'o', 'd' e 'a' com push

Imprima o resultado final

Resultado esperado:

Rust é muito foda*/

fn main() {
    let mut s = String::from("Rust");
    println!("{s}");
    s.push_str(" é muito ");
    s.push('t');
    s.push('o');
    s.push('p');
    s.push('!');

    println!("{s}");
}
