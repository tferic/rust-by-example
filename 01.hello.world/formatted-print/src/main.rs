fn main() {
    // Convert different formats to String
    println!("Integer: {}", 37); 
    println!("Float: {}", 5.5);
    println!("String: {}", "192.168.0.1");

    // positional arguments (numbered) like in awk, perl, python
    println!("Hello {0}, introducing {1}.\nHello {1}, introducing {0}.", "Alice", "Bob");
    
    // positional arguments (named)
    println!("{date}\t{servername}:{port}\t{msg}",
             date = "2023-05-19 21:00:00", 
             servername = "srv1.my.org", 
             port = "443", 
             msg = "GET / HTTP/1.1");

    // generic formatting:
    let x = 4093;
    println!("Formatting {0}: {0} (string)", x);
    println!("Formatting {0}: {0:b} (binary)", x);
    println!("Formatting {0}: {0:o} (octal)", x);
    println!("Formatting {0}: {0:x} (hex lc)", x);
    println!("Formatting {0}: {0:X} (hey uc)", x);
    
    // Justify right, pad with spaces 
    println!("Number is: {:>8}", 1234);

    // Justify right, pad with spaces, named
    println!("Number is: {number:>8}", number=123);

    // Justigy right, pad with spaces, named number argument, named formatting argument
    //  append '$' to named argument, when used as formatter
    println!("Number is: {number:>width$}", number=12345, width=8);

    // Justify right, pad with zeros
    println!("Number is: {:0>8}", 123456);
    println!("Number is: {:08}", 123456);

    //#[allow(dead_code)] // disable `dead_code` which warn against unused module
    #[derive(Debug)]
    struct Structure(i32);
    println!("This struct `{:?}` won't print...", Structure(3));

    let number: f64 = 1.123;
    let width: usize = 12;
    println!("Print surrounding variables directly: {number:>width$}");
    
    // format!() macro works like sprintf()
    let mystr = format!("{:b}", 510);
    println!("512 as bin: {}", mystr);

    // print to STDERR
    eprintln!("WARNING: this is a warning on STDERR!");

    const PI: f32 = 3.141592;
    // Formatting number by precision after floating point
    println!("π is roughly: {:8.3}", PI);
}
