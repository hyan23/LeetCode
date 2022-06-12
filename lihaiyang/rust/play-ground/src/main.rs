

fn main() {
    // let x = 13;
    // println!("{}", x);
    // println!("Hello, world!");

    loop_test();

    println!();

    println!("{}", for_test());

    match_test();

    let r : & 'static str =  "hello world";


}


fn loop_test() -> () {
    let mut x = 0;
    loop {

        print!("{} ", x);

        x = x + 1;

        if x == 100 {
            break;
        }

    }
}

fn for_test() -> i32 {
    let mut result;
    result = 0i32;
    for x in 1..=100 {
        result = result + x as i32;
    }
    result
}

fn match_test() -> () {

    let x = 42;

    match x {
        1 => {
            println!("x is 1")
        }
        2 => {
            println!("x is 2")
        }
        3 | 4 => {
            println!("x is 3 or 4")
        }
        m @ 5..=50 => {
            println!("x is in [5,50], which is {}", m);
        }
        _ => {
            println!("x is {}", x)
        }
    }
    
}


fn struct_test() -> () {
    struct SeaCreature {

        name: String,
        animal_type: String, 
        arms: i32,
        legs: i32,
        weapon: String
    }

    let ferris = SeaCreature{name: String::from("Ferris"), animal_type: String::from("crab"),
        arms: 2, legs: 4, weapon: String::from("claw")};


    let s = String::from("hello world");

    println!("length of {}: {}", s, s.len());



}

