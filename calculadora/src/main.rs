fn main() {
    let operacao = match std::env::args().nth(1) {
        Some(op) => op,
        None => {
            println!("Uso: <operacao> <a> <b>");
            return;
        }
    };
    let a: i32 = match std::env::args().nth(2) {
        Some(num) => match num.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Numero invalido: {}", num);
                return;
            }
        },
        None => {
            println!("Uso: <operacao> <a> <b>");
            return;
        }
    };
    let b: i32 = match std::env::args().nth(3) {
        Some(num) => match num.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Numero invalido: {}", num);
                return;
            }
        },
        None => {
            println!("Uso: <operacao> <a> <b>");
            return;
        }
    };

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