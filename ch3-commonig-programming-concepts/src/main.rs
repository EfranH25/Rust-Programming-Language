const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn sec3_1_vars_muts() {
    // shows how vars & mutability works in rust
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

    println!("Constant val {}", THREE_HOURS_IN_SECONDS);

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    let mut z = 20;
    println!("The value of z is: {z}");
    {
        let z = "cat";
        println!("The value of z is: {z}");
    }
    z = z + 5;
    println!("The value of z is: {z}");
}

fn sec3_2_data_types() {
    let x: i8 = -10;
    let y: u8 = 20;
    println!("i32 integers can be negative: {x}, while u32 integers cannot be negative: {y}");

    let x = -10.1;
    let y: f32 = 20.5;
    println!("f64 float: {x}, f32 float: {y}");
}

fn sec3_3_functions(value: i32, unit_label: char) -> u32 {
    println!("The measurement is: {value}{unit_label}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    let result = format!("{}{}", value.to_string(), unit_label);
    println!("{}", result);

    let z: u32 = 50;
    return z;
}

fn sec3_5_control_flow() {
    let mut counter = 0;
    let result: String = loop {
        counter += 1;
        if counter >= 10 {
            break (counter * 2).to_string();
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("While number: {}", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    // error-prone to use while with index to move through a collections
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    // safer to use for loop instead to move through collections
    for element in a {
        println!("the value in for loop is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    sec3_1_vars_muts();
    println!("========= Done with sec 3.1");
    sec3_2_data_types();
    println!("========= Done with sec 3.2");
    let x = 55;
    let result: u32 = sec3_3_functions(x, 'h');
    println!("Returned result is: {result}");
    println!("========= Done with sec 3.3");
    sec3_5_control_flow();
    println!("========= Done with sec 3.5");
}
