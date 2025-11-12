fn print_type_of<T>(_: &T) {
    println!("O tipo da variável é: {}", std::any::type_name::<T>());
}

fn main() {
    /*
        O shadowing é diferente de mut. Com mut, você está alterando o valor da
        mesma variável. Com shadowing, você está criando uma nova variável com o
        mesmo nome, mas que pode ter um tipo diferente, inclusive.
    */

    println!("\nVariáveis e Imutabilidade em Rust:\n");

    let x = 5;
    println!("O valor imutavel de x é: {}", x);
    // x = 6; // Isso causará um erro de compilação

    let mut y = 6;
    println!("O valor mutavel de y é: {}", y);
    y = 8;
    println!("O novo valor de y é: {}", y);

    println!("\n ---------------------------------------------------- ");

    println!("\nShadowing de Variáveis em Rust:\n");
    let z = 10;
    let z = z + 2; // Shadowing
    let z = z * 4; // Shadowring novamente
    println!("O valor final de z após o shadowing é: {}", z);

    println!(" \n ---------------------------------------------------- ");

    println!("\nExercício Conceitual Rápido:\n");

    /*
        Imagine que você está escrevendo uma função que recebe um nome de
        usuário como uma String. Você precisa:
            1. Remover espaços em branco no início e no fim.
            2. Depois, pegar apenas os 5 primeiros caracteres.

        R: Rust prefere criar novos dados transformados em vez de
        modificar os existentes (um conceito que favorece a segurança), o
        shadowing se torna a ferramenta idiomática para encadear essas
        transformações, especialmente quando o tipo do dado pode mudar no
        processo.
    */

    let username = String::from(" Jose Silva ");
    print_type_of(&username);
    println!("O nome do usuário original é: '{}'", username);
    let username = username.trim();
    let username = &username[0..4];
    print_type_of(&username);
    println!("O nome do usuário processado é: '{}'", username);
}
