/*📝 Exercício 2 — Soma até 100

Usando while, some números naturais crescentes (1, 2, 3, ...) até que a soma chegue a 100 ou mais.
Ao final, imprima a soma e o último número que foi adicionado.*/
fn main(){
    
let mut soma: usize = 0;
let mut last = 0;

while soma <= 100{
    last+=1;
    soma += last;
}
println!("a soma eh: {} e o ultimo numero eh: {}", soma,last);
}