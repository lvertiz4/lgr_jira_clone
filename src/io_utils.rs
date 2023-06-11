use std::io;

pub fn get_user_input() -> String {
    let mut user_input = String::new(); //user_input is a variable that will hold the input from the user input

    io::stdin().read_line(&mut user_input).unwrap();//A handle to the standard input stream of a process. 
    //::stdin() is a function that constructs a new handle to the standard input of the current process. Each handle returned is a reference to a shared global buffer whose access is synchronized via a mutex.
    //1)take input with io::stdin() 2) .read_line() ocks this handle and reads a line of input, appending it to the specified buffer 3) .read_line() returns a Result, from the which value is take using .unwrap(
    user_input
}

pub fn wait_for_key_press(){
    io::stdin().read_line(&mut String::new()).unwrap();
}