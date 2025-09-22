fn main() {
    let operacao = std::env::args().nth(1).unwrap();
    let a: i32 = std::env::args().nth(2).unwrap().parse().unwrap();
    let b: i32 = std::env::args().nth(3).unwrap().parse().unwrap();

    let res = {
        if operacao == "soma" {
            a + b
        }
        else if operacao == "subtrai" {
            a - b
        }
        else if operacao == "multiplica" {
            a * b
        }
        else if operacao == "divide" {
            a / b
        }
        else {
            println!("Operação inválida");
            return;
        }
    };

    println!("Resultado: {}", res);
}