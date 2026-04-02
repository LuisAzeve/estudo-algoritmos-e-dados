//tipos
fn tipos(){

    let x = 5.6; //float
    let y = 1_000_000; //inteiro

    println!("valor de y {}", y);
    println!("valor de x {}", x);

    //tuplas
    let numbers = (1, 2, 3);
    println!("{:?}", numbers.2); //Pra não imprimir tudo na tela se põem ponto(.) pós a chamada da let

    //Array
    let numbers = [1.1, 2.2, 3.3];
    println!("{:?}", numbers[2]); //Pra não imprimir tudo na tela se põem ponto([]) pós a chamada da let
}