struct Retangulo {
    x: f32,
    y: f32,
    largura: f32,
    altura: f32
}

impl Retangulo {
    fn new(x: f32, y:f32, largura:f32, altura:f32) -> Retangulo {
        Retangulo {
            x,
            y,
            largura,
            altura
        }
    }
    fn area(&self) -> f32 {
        self.largura * self.altura
    }
    fn centro(&self) -> (f32, f32) {
        (self.x + self.largura / 2.0, self.y + self.altura / 2.0)
    }
} 

fn main() {
    let ret = Retangulo::new(7.0, 7.0, 2.0, 3.0);
    let area = ret.area();
    let centro = ret.centro();
    println!("Area: {}, Centro: {},{}", area, centro.0, centro.1);
}
