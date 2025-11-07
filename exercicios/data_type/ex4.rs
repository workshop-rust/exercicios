fn main() {
    let mut ponto_flutuante: f32 = 3.14;

    println!("Valor inicial do ponto flutuante (f32): {}", ponto_flutuante);

    ponto_flutuante = "texto"; // erro: troca de tipo (string não pode ser atribuída a f32)

    println!("Valor alterado do ponto flutuante (f32): {}", ponto_flutuante);

    // Como corrigir o erro acima?
}
