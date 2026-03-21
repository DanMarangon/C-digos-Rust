fn main() {
let mut nomes: Vec<&str> = Vec::new();
nomes.push("Nome1");
nomes.push("Nome2");
nomes.push("Nome3");
nomes.push("Nome4");
for nome in &nomes{
println!("{:?}",nome);
}
nomes.pop();
println!("Quantidade de nomes: {:?}",nomes.len());

}
