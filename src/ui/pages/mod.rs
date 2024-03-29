use std::rc::Rc;
use std::any::Any;

use itertools::Itertools;
use anyhow::Result;
use anyhow::anyhow;

use crate::db::JiraDatabase;
use crate::models::Action;

mod page_helpers;
use page_helpers::*;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn as_any(&self) -> &dyn Any;//Any itself can be used to get a TypeId; &dyn Any (a borrowed trait object), it has the is and downcast_ref methods, to test if the contained value is of a given type, and to get a reference to the inner value as a type.
}

pub struct HomePage {
    pub db: Rc<JiraDatabase>,
}

impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        println!("----------------------------- EPICS -----------------------------");
        println!("     id     |               name               |      status      ");

        //TODO: print out epics column contents using get_column_string(). Also make sure epics are sorted by id
        let epics = self.db.read_db()?.epics; //make a copy of epics field from DBState and assign it to the variable 'epics'(remember, DBState is a Struct, and epics is a Hashmap)
        for id in epics.keys().sorted() { //sorted function comes from IterTools module; sorts epics by value of index, which is the 'key' of the epics Hashmap
            let epic = &epics[id];//brackets indicate you are looking for a particular index in the hashmap held in 'epics', which is a copy of DBState::epics
            let id_col = get_column_string(&id.to_string(),11);//11 = twelve space inside id column header (remember: indexes start at zero, so length of 11 = 12 distinct spaces)
            let name_col = get_column_string(&epic.name, 32);
            let status_col = get_column_string(&epic.status.to_string(), 17);
            println!("{} | {} | {}", id_col, name_col, status_col);
        }

        println!();
        println!();

        println!("[q] quit | [c] create epic | [:id:] navigate to epic");
        
        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        //todo!() // match against the user input and return the corresponding action. If the user input was invalid return None
        let epics = self.db.read_db()?.epics;//make a copy of the 'epics' field from the DB State struct. 'epics' is a hashmap of indexed Epics.
        match input {//these are the letters at bottom of the homepage that correspond to fields in Action Enums in 'models.rs'
            "q" => Ok(Some(Action::Exit)),//Returns 'Ok' because function return type is a Result that could return an Option representing an Action enum variant
            "c" => Ok(Some(Action::CreateEpic)),
            input => {
                if let Ok(epic_id) = input.parse::<u32>() {//parse function parses string into another type. With the tubrofish operator, we tell the compiler expect a u32 to be assigned to Ok(epic_id) action
                    if epics.contains_key(&epic_id) { //function from std Hashmap module, returns True if key is found within Hashmap, in this case, the entered epic_id by the user
                        return Ok(Some(Action::NavigateToEpicDetail {epic_id}));
                    }
                }
                Ok(None) //if epic_id entered returns 'False' from .contains_keys function, Result<Option<>> returns 'None', and not a variant of the Actions enum
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct EpicDetail {
    pub epic_id: u32,
    pub db: Rc<JiraDatabase>
}

impl Page for EpicDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let epic = db_state.epics.get(&self.epic_id).ok_or_else(|| anyhow!("could not find epic!"))?;//epic=temp variable to save epic from copy of db_state

        println!("------------------------------ EPIC ------------------------------");
        println!("  id  |     name     |         description         |    status    ");

        //Todo: print out epiocs using get_column_string()
        let id_col = get_column_string(&self.epic_id.to_string(), 5); //&self=EpicDetail struct, 5=width of possible epic id, up to 99,998
        let name_col = get_column_string(&epic.name, 12);//&epic = temp variable to hold epic from db_state variable, which is a reference count of JiraDatabase, a Database trait object
        let desc_col = get_column_string(&epic.description, 27);//'Description' field of Epic struct is a String, and can be printed because String have the Display trait
        let status_col = get_column_string(&epic.status.to_string(), 13);//Status is an Enum, so each variant is converted to a String, which has the Display trait, using to_owned()
        println!("{} | {} | {} | {}", id_col, name_col, desc_col, status_col);

        println!();

        println!("---------------------------- STORIES ----------------------------");
        println!("     id     |               name               |      status      ");        
        //Todo: print out stories using get_column_string(). ALso make sure to sort stories by id
        let stories = &db_state.stories;
        for id in epic.stories.iter().sorted() {
            let story = &stories[id];
            let id_col = get_column_string(&id.to_string(), 11);
            let name_col = get_column_string(&story.name, 32);
            let status_col = get_column_string(&story.status.to_string(), 17);
            println!("{} | {} | {}", id_col, name_col, status_col);
        }
        
        println!();
        println!();

        println!("[p] previous | [u] update epic | [c] create story | [:id:] navigate to story");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        //todo!()//match against the user input and return the corresponding action. If the user input was invalid, return None
        let db_state = self.db.read_db()?;//make a copy of the 'stories' field from the DB State struct. 'stories' is a hashmap of indexed Epics; ? propagates DBState, or an Error, not a Result type
        let stories = db_state.stories;//therefor, if DBState is returned in the last line, and not a Result type, you can reference 'stories' field of DBState struct

        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateEpicStatus {epic_id: self.epic_id})),
            "d" => Ok(Some(Action::DeleteEpic {epic_id: self.epic_id})),
            "c" => Ok(Some(Action::CreateStory {epic_id: self.epic_id})),
            input => {
                if let Ok(story_id) = input.parse::<u32>() {//if the input is a number, match to the 'stories' Vector in the Epic struct
                    if stories.contains_key(&story_id) {// go to next step if there is a match between input number and 'stories' Vector value
                        return Ok(Some(Action::NavigateToStoryDetail {epic_id: self.epic_id, story_id}));
                    }
                }
                Ok(None)
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct StoryDetail {
    pub epic_id: u32,
    pub story_id: u32,
    pub db: Rc<JiraDatabase>
}

impl Page for StoryDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let story = db_state.stories.get(&self.story_id).ok_or_else(|| anyhow!("could not find story!"))?;

        println!("------------------------------ STORY ------------------------------");
        println!("  id  |     name     |         description         |    status    ");

        //Todo: print out story details using get_column_string
        let id_col = get_column_string(&self.story_id.to_string(), 5);//self = StoryDetail struct, that include a user input number and a copy of the Database to see if there is a match with the user input number
        let name_col = get_column_string(&story.name, 12);//here we are referencing the Story struct field
        let desc_col = get_column_string(&story.description, 27);
        let status_col = get_column_string(&story.status.to_string(),13);
        println!("{} | {} | {} | {}", id_col, name_col, desc_col, status_col);


        println!();
        println!();

        println!("[p] previous | [u] update story | [d] delete story");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        //todo!() // match against the user input and return the corresponding action. If the user input was not valid, return none
        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateStoryStatus {story_id: self.story_id})),//self = StoryDetail struct
            "d" => Ok(Some(Action::DeleteStory {epic_id: self.epic_id, story_id: self.story_id})),
            _ => { Ok(None) }
        }
    }

    fn as_any(&self) -> &dyn Any { self}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::test_utils::MockDB};
    use crate::models::{Epic, Story};

    mod home_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase 
                {database: Box::new(MockDB::new())}
            );
            let page = HomePage {db};
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
            let page = HomePage {db};
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new())});

            let epic = Epic::new("".to_owned(), "".to_owned());

            let epic_id = db.create_epic(epic).unwrap();
            
            let page = HomePage {db};

            let q = "q";
            let c = "c";
            let valid_epic_id = epic_id.to_string();
            let invalid_epic_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "q983f2j";
            let input_with_trailing_white_spaces = "q\n";

            assert_eq!(page.handle_input(q).unwrap(), Some(Action::Exit));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateEpic));
            assert_eq!(page.handle_input(&valid_epic_id).unwrap(), Some(Action::NavigateToEpicDetail {epic_id: 1 }));
            assert_eq!(page.handle_input(invalid_epic_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);

        }
    }

    mod epic_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let page = EpicDetail {epic_id, db};
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase{database: Box::new(MockDB::new())});
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let page = EpicDetail {epic_id, db};
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_epic_id() {
            let db = Rc::new(JiraDatabase{database: Box::new(MockDB::new())});
            let page = EpicDetail{epic_id: 999, db};
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase{database: Box::new(MockDB::new())});
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();
            let page = EpicDetail {epic_id, db};

            let p = "p";
            let u = "u";
            let d = "d";
            let c = "c";
            let invalid_story_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(page.handle_input(p).unwrap(), Some(Action::NavigateToPreviousPage));
            assert_eq!(page.handle_input(u).unwrap(), Some(Action::UpdateEpicStatus {epic_id: 1}));
            assert_eq!(page.handle_input(d).unwrap(), Some(Action::DeleteEpic {epic_id: 1}));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateStory {epic_id: 1}));
            assert_eq!(page.handle_input(&story_id.to_string()).unwrap(), Some(Action::NavigateToStoryDetail{epic_id: 1, story_id: 2}));
            assert_eq!(page.handle_input(invalid_story_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);

        }
    }

    mod story_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase { database: Box::new(MockDB::new()) });
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();
            let page = StoryDetail {epic_id, story_id, db};
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();
            let page = StoryDetail { epic_id, story_id, db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_story_id() {
            let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let _ = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();
            let page = StoryDetail {epic_id, story_id:999, db};
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_action() {
            let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
            let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
            let story_id = db.create_story(Story::new("".to_owned(), "".to_owned()), epic_id).unwrap();
            let page = StoryDetail {epic_id, story_id, db};
            let p = "p";
            let u = "u";
            let d = "d";
            let some_number = "1";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(page.handle_input(p).unwrap(), Some(Action::NavigateToPreviousPage));
            assert_eq!(page.handle_input(u).unwrap(), Some(Action::UpdateStoryStatus {story_id}));
            assert_eq!(page.handle_input(d).unwrap(), Some(Action::DeleteStory {epic_id, story_id}));
            assert_eq!(page.handle_input(some_number).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input_with_valid_prefix).unwrap(), None);
            assert_eq!(page.handle_input(input_with_trailing_white_spaces).unwrap(), None);
        }
    }
}