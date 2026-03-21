struct Produto{
nome: String,
preco: f64,
estoque: u32,
}

impl Produto{
fn new(nome: &str)-> Self{
Produto{
nome: String::from(nome),
preco: 10.25,
estoque: 20,
}
}
}
impl Produto{
fn vender(&mut self, qtd: u32){
if qtd > 0 && self.estoque > qtd {
self.estoque -= qtd;
}
else{
self.estoque = self.estoque;
}
}

}
impl Produto{
fn exibir(&self){
println!("Nome: {} preco: {} estoque:{}",self.nome,self.preco,self.estoque);
}
}
fn main() {

let mut banana = Produto::new("Banana");
banana.exibir();
banana.vender(1);
banana.exibir();
}
