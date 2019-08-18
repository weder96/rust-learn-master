use std::io::stdin;
use std::process::Command;
fn main() {
    println!("O jogo vai comecar");
    
    /*Command::new("sleep 2")
        .status()
        .expect("Ocorreu um erro ao executar acao");
    */
    
    Command::new("clear")
        .status()
        .expect("Erro ao limpar a tela");

    let mut range = 1..51;
    let mut guess = String::new();
    let mut points: i8 = 0;
    
    for numbers in range {
        println!("pontos: {}", points);
        println!("Quanto e {} * {} :", numbers, numbers);
        stdin().read_line(&mut guess)
            .expect("Erro ao ler entrada de usuario");
        let mut guess: i32 = guess.trim().parse()
            .expect("Por favor, digite um numero");

        let mut result = numbers * numbers;

        if guess == result {
            println!("Parabens, voce acertou!");
            points += 1;
        } else {
            println!("Voce errou, voce chutou {} mas a resposta era {}", guess, result);
            points -= 1;
        }

        if guess > 10 {
            break;
        }
    }

    if points < 10 {
        println!("Voce perdeu, a meta era 10 pontos e voce so conseguiu {} pontos", points);
    }
}