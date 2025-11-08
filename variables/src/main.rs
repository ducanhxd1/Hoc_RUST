fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 *3;

    let x = 8;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    func_1(10);
    func_2(1, 'a');
}

fn func_1(x: i32) {
    println!("Hi func 2! {x}");
}

fn func_2(value: i32, unit_label: char) {
    println!("The value :{value} {unit_label}");
}

