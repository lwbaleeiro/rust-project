/*
  Tuplas (Tuples)

  Tuplas são uma forma de agrupar um número fixo de valores com diferentes tipos em um único tipo composto. Elas
  são úteis quando você quer combinar alguns valores relacionados sem criar uma estrutura de dados mais complexa.        

   - Características:
       - Tamanho fixo: Uma vez declarada, o número de elementos de uma tupla não pode mudar.
       - Tipos mistos: Os elementos podem ser de tipos diferentes.

  #################################################

  Arrays (Arrays)

  Arrays são coleções de valores do mesmo tipo. Assim como as tuplas, os arrays têm um tamanho fixo e são
  alocados na stack (pilha).

   - Características:
       - Tamanho fixo: Uma vez declarado, o número de elementos de um array não pode mudar.
       - Mesmo tipo: Todos os elementos devem ser do mesmo tipo.
*/

/*
  Importante: Em Rust, arrays são de tamanho fixo. Se você precisa de uma coleção que possa crescer ou diminuir
  de tamanho, você usaria um Vec (Vector), que é um tipo de coleção dinâmica. Veremos Vec em um tópico futuro.
*/

fn main() {
    
    println!("\n ######## Tuplas ######## \n");

    let pessoa: (&str, i32, bool) = ("Alice", 30, true);
    let (nome, idade, ativo) = pessoa;

    println!("Acessando elemento por desestruturação:");
    println!("Nome: {}, Idade: {}, Ativo: {}", nome, idade, ativo);

    println!("\nAcessando elemento por índice:");
    println!("Nome: {}", pessoa.0);
    println!("Idade: {}", pessoa.1);
    println!("Status de ativo é: {}", pessoa.2);

    println!(" \n ---------------------------------------------------- ");

    println!("\n ######## Arrays ######## \n");
    
    let numeros: [i32; 5] = [10, 20, 30, 40, 50];

    println!("\nAcessando elementos por índice:");
    println!("O primeiro número é: {}", numeros[0]);
    println!("O terceiro número é: {}", numeros[2]);
    
    println!("\nInicializando um array com valores repetidos:");
    let zeros = [0; 5];
    println!("Array de zeros: {:?}", zeros);
}
