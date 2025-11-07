/*📝 Exercício 1 — Contagem até 5

Crie um loop que começa de 1 e imprime os números até 5.
Use break para interromper a execução quando o valor chegar a 5.*/

fn main(){

    let mut contador = 0;
    loop{
        contador +=1;
        println!("{}", contador);
        if contador == 5 {
            break;
        }
    }
}