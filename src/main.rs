extern crate rand;


use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Adivinhe o numero!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    let mut tentativas = 0;

loop { 
    println! ("digite o seu palpite.");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)
        .expect("falha ao ler entrada");
    
    let palpite: u32 = match palpite.trim().parse() {
        Ok(num) => num,
        Err(_) => { 
            println!("Por favor, digite apenas numeros!");
            continue; }
         };

    println!("voce disse: {}", palpite);

    tentativas += 1;

    let distancia = palpite.abs_diff(numero_secreto);
        
    match palpite.cmp(&numero_secreto) {
        Ordering::Less => {
            if distancia <= 5 {
                println!("Muito baixo, mas tá pegando fogo!");
            } else {
                println!("Muito baixo!");
            }
        }
        Ordering::Greater =>{
            if distancia <= 5{
                println!("Muito alto, mas tá pegando fogo!");
            } else {
                println!("Muito alto!")
            }
        }
        Ordering::Equal => {
            println!("Você acertou em {} tentativas!", tentativas);
            break;
        }
    }

  
        if tentativas >= 5 {
            println!("Game over! o número secreto era {}", numero_secreto);
            break;
        }
    }
}