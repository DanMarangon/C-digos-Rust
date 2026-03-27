struct Livro
{
titulo: String,
autor: String,
paginas: u32
}
impl Livro{
fn eh_longo(&self) -> bool{
 if self.paginas > 300{
    return true
}
else {
return false
}
}
}
impl Livro{
 fn apresentar(&self){
println!("Titulo: {:?} Autor: {:?} Paginas: {:?}", self.titulo, self.autor,self.paginas);

}
}
fn main(){
let mut livro1 = Livro{
titulo: String::from("Roblox"),
autor: String::from("Dan"),
paginas: 301
};
println!("{:?}",livro1.eh_longo());
livro1.apresentar();
}
