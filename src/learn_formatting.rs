pub(crate) fn tryformatting() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    // Formatting
    let x = 5 + /*some comment*/ 5;
    let y = "asdadadasdasd";
    println!("is `x` 10 or 100? x = {} and y = {}", x, y);
    println!("now another formatting {1} now x {0}", x, y);
    println!("named formatting {name} {age} {place}", place="Bangalore", name="Rajesh", age=30);
    println!("Different formating base 10 {0} hexa {0:x} octal {0:o} Binary {0:b}", x);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:_>width$}");
}