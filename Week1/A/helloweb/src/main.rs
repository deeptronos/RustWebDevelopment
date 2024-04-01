fn print_msg(msg: &str) -> usize{
    println!("{}", msg);
    msg.len()
}

fn main() {
    let len = print_msg("hello world!");
    println!("{}", len);
}
