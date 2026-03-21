fn main() {
let ponto: (f64,f64) = (10.1,20.2);
let (x,y) = ponto;
let cidades: [&str;4] = ["cidade1","cidade2","cidade3","cidade4"];
println!("{:?}",ponto);
println!("{}{}{}{}",cidades[0],cidades[1],cidades[2],cidades[3]);


}
