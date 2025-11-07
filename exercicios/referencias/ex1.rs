//referencia imutavel 

fn main(){
    let x = 10;
    let ref_x = &x;
    let ref_ref_x = &ref_x;
    println!("O valor de x é: {}, o valor referenciado por ref_x é: {}, o valor referenciado por ref_ref_x é: {}", x, ref_x, ref_ref_x);
}