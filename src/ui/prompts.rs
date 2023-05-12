use crate::{models::{Epic, Story, Status}, io_utils::get_user_input};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() ->Option<Status>>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("----------------------------");
    println!("Epic Name: ");
    let epic_name = get_user_input();//create variable to store first user input
    println!("Epic Description: ");
    let epic_desc = get_user_input();
    let epic = Epic::new(epic_name.trim().to_string(), epic_desc.trim().to_owned());//instantiate new Epic with user input assigned to particular struct fields
    epic
}

fn create_story_prompt() -> Story {
 println!("----------------------------");
 println!("Story Name: ");
 let story_name = get_user_input();
 println!("Story Description: ");
 let story_desc = get_user_input();
 let story = Story::new(story_name.trim().to_string(), story_desc.trim().to_owned());
 story

}
fn delete_epic_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/N]:");
    let input = get_user_input();
    if input.trim().eq("Y") {//.eq() method tests for self and other values to be equal, and is used by ==

        return true;
    }
    false
}

fn delete_story_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this story? [Y/N]:");
    let input = get_user_input();
    if input.trim().eq("Y") {
        return true;
    }
    false
}
fn update_status_prompt() -> Option<Status>{
    prinln!("----------------------------");
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED");
    let input = get_user_input();
    let input = input.trim().parse::<u8>(); //prase function turns &str into u8 integers with tuberfish syntax
    match input {
        1 => {
            return Some(Status::Open);
        }
        2 => {
            return Some(Status::InProgress);
        }  
        3 => {
            return Some(Status::Resolved);
        }
        4 => {
            return Some(Status::Closed);
        }
    }
    None //Don't forget to return None with a Option
}