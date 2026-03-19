
fn cumprimentar(nome: &String){
println!("o nome é: {nome}");}

fn main() {
let nomee = String::from("Dan");
cumprimentar(&nomee);
println!("{nomee}");

}
