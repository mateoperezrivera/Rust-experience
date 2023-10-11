fn main(){
    let args: Vec<String> = env::args().collect();
    let program_name=&args[0];
    let objects_map=create_map(program_name);
}
