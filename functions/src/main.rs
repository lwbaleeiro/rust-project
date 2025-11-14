/*
  Em Rust, as funções são declaradas usando a palavra-chave fn. Elas podem receber parâmetros e retornar
  valores. 
*/

fn minha_funcao() {
    // Declaração simples
    println!("Chamando função minha_funcao");
}

fn cumprimentar(nome: &str) {
    // Declaração com parâmetro
    println!("Olá, {}!", nome)
}

fn somar(a: i32, b: i32) -> i32 {
    // Declaração com parâmetros e valor de retorno
    a + b
}

fn somar_explicito(a: i32, b: i32) -> i32 {
    // Declaração com retorno explícito usando return
    return a + b;
}

fn condicoes() {
    // Uso comum do statement
    let numero = 7;
    if numero < 5 {
        println!("Condição verdadeira, o numero é menor do que 5");
    } else {
        println!("Condição falsa, o numero é maior ou igual a 5");
    }

    // Agora como expressão (não é necessário passar ; no 'result')
    let numero_two = 3;
    let resultado = if numero_two < 5 {
        "menor que 5"
    } else {
        "maior que 5"
    };

    println!("Resultado como expressão é: {}", resultado); 
}

fn repeticoes() {

    // Loop normal
    let mut contador = 0;
    loop {
        println!("Loop infinito! Contador: {}", contador);
        contador += 1;

        if contador == 9 {
            break;
        }
    } 
    println!("Saímos do loop!");

    // Loop como expressão
    let mut contador_dois = 0;
    let resultado = loop {
        contador_dois += 1;
        if contador_dois == 10 {
            break contador_dois * 2;
        }
    };
    println!("O resultado de loop é: {}", resultado);

    // while loop
    let mut numero = 5;
    while numero != 0 {
        println!("{}!", numero);
        numero -= 1;
    }
    println!("LIFTOFF!!!");

    // For loop
    for numero_dois in 1..5 {
        // o numero 5 vai ficar de fora do loop
        println!("O numero do for atual é {}", numero_dois);
    }

    for numero_tres in 1..=5 {
        // o numero 5 vai ser incluido no loop
        println!("O numero inclusivo atual é {}", numero_tres);
    }

    // For em uma coleção
    let a = [10,20,30,40,50];
    for elemento in a.iter() {
        println!("O valor é da coleção no loop é: {}", elemento);
    }
}

fn main() {
    minha_funcao();
    cumprimentar("Alice");
    println!("{}", somar(03, 79));
    println!("{}", somar_explicito(133, 56));


    // Confições
    condicoes();

    // Repetições
    repeticoes();
}
