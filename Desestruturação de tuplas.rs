fn main()
{
let filme = ("Dragão",2025);
println!("{:?}",filme);

println!("{:?}",filme.0);
println!("{:?}",filme.1);

let (titulo,ano) = filme;

println!("Título: {:?}, Ano: {:?}", titulo,ano);


}
