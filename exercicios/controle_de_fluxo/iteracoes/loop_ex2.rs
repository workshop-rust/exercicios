/*📝 Exercício 2 — Somatório com loop retornando valor

Use um loop para somar números inteiros começando de 1 até que a soma chegue ou ultrapasse 50.
Quando isso acontecer, use break para sair do loop, retornando o valor final da soma.

Depois, imprima o valor retornado.*/

fn main(){

    let mut soma = 0;
    let resultado = loop{
        soma +=1;
        if soma >=50{
            break soma;
        }
    }
    println!("O valor final da soma é: {}", resultado);
}