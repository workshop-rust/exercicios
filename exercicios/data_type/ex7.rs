fn main() {
    let mut string_slice: &str = "Hello, Rust!";

    println!("Valor inicial do string slice: {}", string_slice);

    string_slice = String::from("Nova String"); // erro: String não pode ser atribuída a &str

    println!("Valor alterado do string slice: {}", string_slice);

    // Como corrigir o erro acima?
}
