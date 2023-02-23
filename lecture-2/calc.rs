use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Error: Wrong number of arguments");
        std::process::exit(-1);
    }

    let operator = args[2].to_string();
    let num_a = args[1].parse::<f32>().unwrap();
    let num_b = args[3].parse::<f32>().unwrap();

    let r: f32 = calculate(&num_a, &num_b, &operator);

    println!("{} {} {} = {}", num_a, operator, num_b, r);
}

fn calculate(a: &f32, b: &f32, op: &String) -> f32 {
    let mut result: f32 = 0.0;

    match op.as_ref() {
        "+" => result = a + b,
        "-" => result = a - b,
        "x" => result = a * b,
        "/" => result = a / b,
        _ => println!("Unknown Operator"),
    }
    return result;
}
