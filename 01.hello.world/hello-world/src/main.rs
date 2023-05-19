fn main() {
    // println!() is a marcro that prints on STDOUT and includes a newline
    println!("Hello, world!");

    /* print!() is a macro
       that prints on STDOUT
       and does not add a newline */
    print!("Hello");
    print!(" ");
    print!("World!\n");
    
    let x = 5 + /* wow, inline comments */   5;
    
    // the {} in a print statement converts any type to String
    println!("The result of the math operation is: {}", x );
}
