// desenvolver um código para saber se um número é positivo, negativo ou zero.

fn main() {
    let numero = 7;

    if numero < 0 {
        println!("O número é negativo.");
    } else if numero == 0 {
        println!("O número é zero.");
    } else {
        println!("O número é positivo.");
    }
}