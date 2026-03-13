use std::{fs::File, io::ErrorKind, io::Read, io};
use std::error::Error;


fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e)
    // };
    // Question mark handles match logic of result similar as above code (how error is returned is different)
    let mut username_file = username_file_result?;

    let mut username = String::new();
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }
    username_file.read_to_string(&mut username)?;
    Ok(username)

    // even simpler way via chaining:
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
fn main() -> Result<(),Box< dyn Error>>{
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


    let greeting_file = File::open("hello.txt")?;

    Ok(())

}
