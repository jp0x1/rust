fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn main() {
    println!("Hello, world!");

    another_function(5, 'k');
    let x = five();
    println!("{x}");
    let x = plus_one(3);
    println!("{x}");
}

fn another_function(x: i32, unit_label: char) {
    println!("Value of x is: {x}{unit_label}");
}