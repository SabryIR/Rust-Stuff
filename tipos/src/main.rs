#![allow(dead_code)]

#[derive(Debug)]
enum Semaforo {
    Vermelho,
    Amarelo,
    Verde
}

#[derive(Debug)]
enum Naipe {
    Ouros,
    Copas,
    Paus,
    Espadas
}

#[derive(Debug)]
enum Rank {
    A,
    Numero(u8),
    Q,
    J,
    K
}

#[derive(Debug)]
struct Carta {
    naipe: Naipe,
    rank: Rank
}

#[derive(Debug)]
struct Ponto {
    x: f32,
    y: f32
}

#[derive(Debug)]
enum ObjGeometrico {
    Ponto(Ponto),
    Semirreta {
        origem: Ponto,
        angulo: f32
    },
    Segmento(Ponto, Ponto),
    Quadrilatero([Ponto; 4])
}

#[derive(Debug)]
struct Livro {
    titulo: String,
    autor: String,
    paginas: u16
}

#[derive(Debug)]
struct Revista {
    titulo: String,
    edicao: u8
}

#[derive(Debug)]
enum ItemBiblioteca {
    Livro(Livro),
    Revista(Revista)
}

fn main() {
    let semaforo = Semaforo::Verde;
    let as_espadas = Carta {
        naipe: Naipe::Espadas,
        rank: Rank::A,
    };
    let ponto = ObjGeometrico::Ponto(Ponto {
        x: 2.0,
        y: 3.0
    });
    let quad = ObjGeometrico::Quadrilatero([
        Ponto {
            x: 1.0,
            y: 1.0,
        },
        Ponto {
            x: 2.0,
            y: 1.0,
        },
        Ponto {
            x: 1.0,
            y: 2.0,
        },
        Ponto {
            x: 2.0,
            y: 2.0,
        },
    ]);
    let livro = ItemBiblioteca::Livro(Livro {
        titulo: String::from("Crime e Castigo"),
        autor: String::from("Fiodor Dostoievski"),
        paginas: 562
    });
    println!("{:?}", semaforo);
    println!("{:?}", as_espadas);
    println!("{:?}", ponto);
    println!("{:?}", quad);
    println!("{:?}", livro);
}
