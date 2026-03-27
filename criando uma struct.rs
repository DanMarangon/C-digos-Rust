fn main(){
struct Livro{
titulo: String,
autor: String,
paginas: u32,
}
let livro1 = Livro{
titulo: String::from("Roblox"),
paginas: 300,
autor: String::from("Dan")
};
println!("[{:?}] por [{:?}] - [{:?}]", livro1.titulo,livro1.autor,livro1.paginas);

}
