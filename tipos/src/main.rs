#![allow(dead_code)]

#[derive(Debug)]
enum Semaforo {
    Vermelho,
    Amarelo,
    Verde
}

#[derive(Debug)]
enum Naipe {
    Copas,
    Espadas,
    Ouros,
    Paus,
}

#[derive(Clone, Debug)]
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

impl Carta {
    fn manilhas(&self) -> [Carta; 4] {
        let rank = match self.rank {
            Rank::Numero(7) => Rank::Q,
            Rank::Numero(n) => Rank::Numero(n+1),
            Rank::Q => Rank::J,
            Rank::J => Rank::K,
            Rank::K => Rank::A,
            Rank::A => Rank::Numero(2)
        };

        [
            Carta {
                naipe: Naipe::Copas,
                rank: rank.clone()
            },
            Carta {
                naipe: Naipe::Espadas,
                rank: rank.clone()
            },
            Carta {
                naipe: Naipe::Ouros,
                rank: rank.clone()
            },
            Carta {
                naipe: Naipe::Paus,
                rank: rank.clone()
            }
        ]
    }
}

fn main() {
    let vira = Carta {
        naipe: Naipe::Espadas,
        rank: Rank::A,
    };

    let man = vira.manilhas();
    println!("{:?}", man);
}
