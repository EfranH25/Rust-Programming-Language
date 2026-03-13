use std::{fs::File, io::ErrorKind};

fn main() {
    let v = vec![1, 2, 3];

    // when reading backtrace --> lines before spot where code failed are lines your code called
    // v[99];
    // lines after panic occurred are lines of code that called your code

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {e:?}"),
        },
        _ => {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // built in method to give back result if Result is 'ok'
    let greeting_file = File::open("hello.txt").unwrap();

    // built in method similar to unwrap but lets you set the panic! message
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    // in production most use .expect instead of .unwrap b/c it gives more context
}
