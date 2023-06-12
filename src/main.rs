use std::rc::Rc;

mod models;
mod db;
use db::*;

mod ui;
mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;


fn main() {
    //TODO: create database and manager
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_owned()));
    let mut navigator = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        //TODO: implemnt the following functionality
        //1. get current page from the Navigator
        if let Some(page) = navigator.get_current_page() {//page is a Box pointer to any object that contains the Page trait, inferred by call to get_current_page() method for Navigator struct; recall box pointers allocate values on the heap
            //2. render page
            if let Err(error) = page.draw_page() {
                println!("Error rendering page: {}\nPress any key to continue...", error);
                wait_for_key_press();
            };
        //3. get user input
        let user_input = get_user_input();

          //4. pass input to page's input handler    
        match page.handle_input(user_input.trim()) { //handle_input is a method for Navigator struct; therefore compiler infers page relates to pages field of Navigator struct, which has Page trait objects in a vector
            Err(error) => {
                println!("Error getting user input: {}\nPress any key to continue...", error);
                wait_for_key_press();
            },
            //5. if the page's input handler returns an action let the navigator process the action
            Ok(action) => {
                if let Some(action) = action {
                    if let Err(error) = navigator.handle_action(action) {
                        println!("Error handling processing user input: {}\nPress any key to continue...", error);
                        wait_for_key_press();
                    }
                }
            }
        }
    } else {
        break;//recall that 'break' is a keyword to exit early from a loop, in this case, the loop to render the screen
        }
    }
}