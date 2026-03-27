fn main(){
let nota = 85;

match nota{
90..=100 => println!("Excelente"),
70..=89 => println!("Aprovado"),
50..=69 => println!("Recuperação"),
0..=49 => println!("Reprovado"),
_ => println!("Número Invalido"),

}
let dia = 6;
match dia{
1|2|3|4|5 => println!("Dia útil"),
6|7 => println!("Fim de semana"),
_ => println!("Dia inválido"),
}
}
