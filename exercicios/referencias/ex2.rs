// referencia mutavel 

fn main() {
    let mut x = 10;
    {
        let r = &mut x; // ← empresta x mutavelmente
        *r += 5;        // ✅ ok: modifica via referência
        println!("{}", r); 
        println!("{}", x); // ❌ erro
        println!("{}", r);
        x = 30;            // ❌ erro
    } // referência r termina aqui
    println!("{x}"); // ✅ ok
}