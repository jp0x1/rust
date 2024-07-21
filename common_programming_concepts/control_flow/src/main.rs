fn for_loop(){
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn better_for_loop(){
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("Lift off!!");
    for number in 1..4 {
        println!("{number}");
    }
}

fn while_loop(){
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFT OFF WITH WHILE LOOP!");
}


fn main() {
    for_loop();
    println!("Hello, world!");
}
