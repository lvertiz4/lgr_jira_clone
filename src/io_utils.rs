use std::io;

pub fn get_user_input() -> String {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();//:stdin() is a function that constructs a new handle to the standard input of the current process.Each handle returned is a reference to a shared global buffer whose access is synchronized via a mutex.

    user_input
}