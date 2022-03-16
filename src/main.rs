use std::env::args;

//Exercice 1 & 2
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

// Exercice 3
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn sub(a: i32, b: i32) -> i32 {
    a - b
}
fn mul(a: i32, b: i32) -> i32 {
    a * b
}
fn rem(a: i32, b: i32) -> i32 {
    a % b
}
fn cat(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    //Exercice 1 main
    let a = 5;
    let b = 2;

    match div(a, b) {
        None => println!("Nu se poate face impartirea"),
        Some(rez) => println!("Rezultatul impartirii celor doua numere este: {}", rez),
    }

    //Exercice 2 main
    let params: Vec<String> = args().collect();
    let x: i32 = match params[2].parse() {
        Ok(x) => x,
        Err(_) => std::process::exit(-1),
    };
    let y: i32 = match params[3].parse() {
        Ok(y) => y,
        Err(_) => std::process::exit(-1),
    };

    match div(x, y) {
        None => std::process::exit(-1),
        Some(rez) => println!("Rezultatul impartirii celor doua numere este: {}", rez),
    }

    //Exercice 3 main
    let o = params[1].as_str();
    match o {
        "add" => println!("adunare: {} ", add(x, y)),
        "sub" => println!("scadere: {}", sub(x, y)),
        "div" => println!("catul impartirii: {}", cat(x, y)),
        "mul" => println!("inmultire: {}", mul(x, y)),
        "mod" => println!("mod impartirii: {}", rem(x, y)),
        _ => std::process::exit(-1),
    }

    //Exercice 4
    let foo;
    match env::var("CMD") {
        Ok(val) => foo = val,
        Err(_e) => foo = "none".to_string(),
    }
}
