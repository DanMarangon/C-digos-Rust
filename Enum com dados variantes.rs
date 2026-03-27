enum Forma
{
Circulo(f64),
Retangulo(f64,f64),
Quadrado(f64),
}
fn descrever(f: Forma){
match f{
Forma::Circulo(raio) => println!("O raio do círculo é: {:?}",raio),
Forma::Retangulo(lado,altura) => println!("O lado do retângulo é: {:?} e sua altura é: {:?}",lado,altura),
Forma::Quadrado(ladoo) => println!("O lado do quadrado é: {:?}",ladoo),
}
}

fn main() {
descrever(Forma::Circulo(1.1));
descrever(Forma::Retangulo(2.2,3.3));
descrever(Forma::Quadrado(10.2));


}
