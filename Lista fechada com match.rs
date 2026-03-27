#[derive(Debug)] enum Clima{ Ensolarado,Nublado,Chuvoso};
fn main(){
let hoje = Clima::Chuvoso;

match hoje{
Clima::Ensolarado => println!("O clima está Ensolarado"),
Clima::Nublado => println!("O clima está Nublado"),
Clima::Chuvoso => println!("O clima está chuvoso"),
}
println!("{:?}",hoje);
}
