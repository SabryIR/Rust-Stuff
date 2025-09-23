// exercicio incompleto

#[allow(dead_code)]
enum Semaforo {
    Vermelho,
    Amarelo,
    Verde
}

#[allow(dead_code)]
enum Naipe {
    Ouros,
    Copas,
    Paus,
    Espadas
}

#[allow(dead_code)]
enum Rank {
    A,
    Numero(u8),
    J,
    K,
    Q
}

#[allow(dead_code)]
struct Carta {
    naipe: Naipe,
    rank: Rank
}

fn dizer_ola() {
    println!("Olá!");
}

fn main() {
    dizer_ola();
}
