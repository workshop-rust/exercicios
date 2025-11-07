
fn main(){
    let inteiro_com_sinal : i8 = -10;
    let inteiro_sem_sinal : u8 = 10;
    let ponto_flutuante : f32 = 3.14;   
    let booleano : bool = true;
    let caractere : char = 'R';
    let string_slice : &str = "Hello, Rust!";
    let string_heap : String = String::from("Hello, Heap!");


    println!("Inteiro com sinal (i8): {}", inteiro_com_sinal);
    println!("Inteiro sem sinal (u8): {}", inteiro_sem_sinal);
    println!("Ponto flutuante (f32): {}", ponto_flutuante);
    println!("Booleano (bool): {}", booleano);
    println!("Caractere (char): {}", caractere);
    println!("String slice (&str): {}", string_slice);
    println!("String na heap (String): {}", string_heap);
    
}