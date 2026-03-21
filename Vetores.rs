fn main() {
let mut tarefas: Vec<&str> = Vec::new();
tarefas.push("Um");
tarefas.push("Dois");
tarefas.push("Três");

println!("{}",tarefas.len());

for tarefa in &tarefas{
println!("{}",tarefa);

}
tarefas.pop();
println!("{}",tarefas.len());
}
