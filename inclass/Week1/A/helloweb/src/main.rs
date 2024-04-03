

fn print_msg(msg: &str) -> usize{
    println!("{}", msg);
    msg.len()
}

fn if_ex(){
    let x = 5;
    let y = if x > 0{
        "hello"
    } else {
        "goodbye"
    };

    let  x = if x == 5{ // Acceptable form.
        "five"
    } else {
        "?"
    };

    println!("x: {}", x);
    println!("y: {}", y);
}


fn for_loop_ex(num : u8){ // Print numbers from 1 to num, in reverse order. Accepts unsigned 8bit integer.
    for i in (1u8 ..= num).rev() { // Also works over collections ... brings up some unrelated questions of ownership...
        println!("{}", i);
    }
}



fn main() {
    let len = print_msg("hello world!");
    println!("{}", len);
    if_ex();
    for_loop_ex(10);
}
