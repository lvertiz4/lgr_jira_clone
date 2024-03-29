use anyhow::{Result, anyhow};
use std::fs;
use crate::models::{DBState, Epic, Story, Status};

pub struct JiraDatabase {
    pub database: Box<dyn Database>
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        Self {
            database: Box::new(JSONFileDatabase{
                file_path
            }) //creates new instance of JSONFileDatabase, which implements the Database traits 'read' and 'write', using file_path of the location of DBState
        }
    }    
    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db() //returns a copy/instance of DBState
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut parsed = self.database.read_db()?;
        let last_id = parsed.last_item_id;
        let new_id = last_id + 1;
        parsed.last_item_id = new_id;
        parsed.epics.insert(new_id, epic); //takes epic argument, and new_id to create new epic in this instance of DBState, 'parsed'
        self.database.write_db(&parsed)?; //take 'parsed', now with new epic included, to write over the DBState instance 'self' - i.e. the original DBState
        Ok(new_id) //confirm write to DBState successful by return new_id of newly-uploaded epic
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut parsed = self.database.read_db()?; //create parsed instance of DBState
        let last_id = parsed.last_item_id;
        let new_id = last_id + 1; //create new id for story

        parsed.last_item_id = new_id;
        parsed.stories.insert(new_id, story); //add Story struct into DBState using newly-generated new_id

        parsed.epics.get_mut(&epic_id).ok_or_else(|| anyhow!("Couldn't find Epic in database!"))?.stories.push(new_id); // add Story's new_id to Epic's stories field, which contains a Vector of u32 of Story ID's

        self.database.write_db(&parsed)?; //write new ID, story struct and Story ID within Epic's Story field, to the parased instance of DBState
        Ok(new_id) //return new_id value to confirm function that wrote new values to parsed instance of DBState

    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> { //***When you delete an Epic, you also delete all linked Storys in DBState Hashmap of stories */
        let mut parsed = self.database.read_db()?; //create mut instance of DBState struct within 'parsed' variable
        
        for story_id in &parsed.epics.get(&epic_id).ok_or_else(|| anyhow!("could not find epic in database!"))?.stories { // within the Epic struct of DBState, for each element within the Story field, which is a Vector of u32 unique ID's
            parsed.stories.remove(story_id); //remove all stories using the Story IDs from the Epic struct to remove individual Storys from Stories Hashmap of DBState
        }

        parsed.epics.remove(&epic_id); //remove epic from parsed instance of DBState with epic_id from provided the function arguments' parameters

        self.database.write_db(&parsed)?;
        Ok(())
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        let mut parsed = self.database.read_db()?;

        let epic = parsed.epics.get_mut(&epic_id).ok_or_else(|| anyhow!("count not find epic in database"))?;
        let story_index = epic.stories.iter().position(|id| id == &story_id).ok_or_else(|| anyhow!("story id not found in epic's stories vector"))?;// from within Epic struct, the stories field has a Vector of u32, returning index of Story with provided story_id.


        epic.stories.remove(story_index); // within Epic struct (chose from epics hashmap using provided epic_id) from parsed DBState instance, remove story_id based on story_id provided in function argument's parameters

        parsed.stories.remove(&story_id);//within parsed DBState instance, remove Story structs from stories HashMap using provided story_id

        self.database.write_db(&parsed)?;
        Ok(())
    }

    pub fn update_epic_status(&self, epic_id:u32, status: Status) -> Result<()> {
        let mut parsed = self.database.read_db()?;
        parsed.epics.get_mut(&epic_id).ok_or_else( || anyhow!("Could not find epic in database!"))?.status = status;
        self.database.write_db(&parsed)?;
        Ok(())
    }

    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut parsed = self.database.read_db()?;
        parsed.stories.get_mut(&story_id).ok_or_else( || anyhow!("Could not find story in database!"))?.status = status;
        self.database.write_db(&parsed)?;
        Ok(())
    }
}

pub trait Database {

    fn read_db(&self) -> Result<DBState>;

    fn write_db(&self, db_state: &DBState) -> Result<()>;

}

struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {

    fn read_db(&self) -> Result<DBState> {
      let content = fs::read_to_string(&self.file_path)?; // read the content's of self.file_path 
      let parsed: DBState = serde_json::from_str(&content)?; //deserialize content variable using serde_json
      Ok(parsed)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        fs::write(&self.file_path, serde_json::to_vec(&db_state)?)?; // serialize db_state to json and store it in self.file_path
        Ok(())
    }
}

pub mod test_utils {
    use std::{cell::RefCell, collections::HashMap};

    use super::*;
    pub struct MockDB {
        last_written_state: RefCell<DBState>, //single owner of DBState struct
    }

    impl MockDB {
        pub fn new() -> Self { //instantiate a new instance of DBState called MockDB; 
                               // remember to instantiate new MockDB struct as a RefCell (i.e. single owner, but field inside struct are mutable/writeable, even if they are referenced by someone else),
                               // then copy code/instructions to instantiate a new, clean DBState struct called MockDB
            Self {
                last_written_state: RefCell::new(DBState{last_item_id: 0, epics: HashMap::new(), stories: HashMap::new() })
            }
        }
    }

    impl Database for MockDB {
        fn read_db(&self) -> Result<DBState> {
            //TODO: Fix this error by deriving the appropriate trais for DBState
            //Answer: added 'Clone' attribute, to allow 'state' to make a copy of DBState inside the RefCell smart pointer
            let state = self.last_written_state.borrow().clone(); //Self=MockDB; .borrow() immutably (i.e. reads) the value inside RefCell, which is an instance of DState
            //.clone() is called to make a copy of the inside of RefCell, which is a new instance of DBState
            Ok(state) // the new instance of DBState, within the instance of MockDB, is returns from the function call
        }

        fn write_db(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state; //one owner of DBState struct, which is now variable 'latest_state'
            //TODO:fix this error by deriving the appropriate traits for DBState
            //Answer: added 'Clone' attribute, to allow 'state' to make a copy of DBState inside the RefCell smart pointer
            *latest_state.borrow_mut() = db_state.clone(); //mutably/write borrows DBState inside RefCell, and copies that value to be altered later
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_utils::MockDB;

    #[test]
    fn create_epic_should_work() {
        let db = JiraDatabase{
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        //TODO: fix this error by deriving the appropr
        let result = db.create_epic(epic.clone());

        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 1;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&id), Some(&epic));
    }

    #[test]
    fn create_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase{
            database: Box::new(MockDB::new())
        };
        let story = Story::new("".to_owned(), "".to_owned());
        
        let non_existent_epic_id = 999;
        let result = db.create_story(story, non_existent_epic_id);
        assert_eq!(result.is_err(), true);

    }

    #[test]
    fn create_story_should_work() {
        let db = JiraDatabase{
            database: Box::new(MockDB::new())
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();
        //TODO: fix this error by deriving the appropriate traits for the Story struct
        let result = db.create_story(story.clone(), epic_id.clone());
        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 2;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.contains(&id), true);
        assert_eq!(db_state.stories.get(&id), Some(&story));
    }

    #[test]
    fn delete_epic_should_error_if_invalid_epic_id() {
        let db = JiraDatabase{
            database: Box::new(MockDB::new()),
        };
        let non_existent_epic_id = 999;

        let result = db.delete_epic(non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_epic_should_work() {
        let db = JiraDatabase{
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);
        
        let epic_id = result.unwrap();
        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();
        let result = db.delete_epic(epic_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();
        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id), None);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn delete_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase{
            database: Box::new(MockDB::new())
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());
        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();
        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();
        let non_existent_epic_id = 999;
        let result = db.delete_story(non_existent_epic_id, story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_error_if_story_not_found_in_epic() {
        let db = JiraDatabase{
            database: Box::new(MockDB::new())
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());
        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();
        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let non_existent_story_id = 999;
        let result = db.delete_story(epic_id, non_existent_story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_work() {
        let db = JiraDatabase{
            database: Box::new(MockDB::new())
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());
        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();
        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();
        let result = db.delete_story(epic_id, story_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();
        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.contains(&story_id), false);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn update_epic_status_should_error_if_invalid_epic_id() {
        let db = JiraDatabase{
            database: Box::new(MockDB::new()),
        };
        let non_existent_epic_id = 999;
        let result = db.update_epic_status(non_existent_epic_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_epic_status_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();
        let result = db.update_epic_status(epic_id, Status::Closed);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();
        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
    }

    #[test]
    fn update_story_status_should_error_if_invalid_story_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());
        let result = db.create_epic(epic);
        let epic_id = result.unwrap();
        let result = db.create_story(story, epic_id);
        let story_id = result.unwrap();
        let result = db.update_story_status(story_id, Status::Closed);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();
        assert_eq!(db_state.stories.get(&story_id).unwrap().status, Status::Closed);
    }

    mod database {
        use std::collections::HashMap;
        use std::fs::{remove_file};
        use std::io::Write;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase {
                file_path: "INVALID_PATH".to_owned(),
            };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let file_path = "./data/read_db_should_fail_with_invalid_json.json".to_owned();

            let path = tmpfile.into_temp_path();
            path.persist(&file_path).unwrap();
            
            let db = JSONFileDatabase{
                file_path: file_path.clone()
            };

            let result = db.read_db();
            
            remove_file(file_path).unwrap();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let file_path = "./data/write_db_should_work.json".to_owned();

            let path = tmpfile.into_temp_path();
            path.persist(&file_path).unwrap();

            let db = JSONFileDatabase {
                file_path: file_path.clone()
            };

            let story = Story {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open
            };
            let epic = Epic {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open,
                stories: vec![2]
            };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState{
                last_item_id: 2,
                epics,
                stories
            };

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            remove_file(file_path).unwrap();

            assert_eq!(write_result.is_ok(), true);
            //TODO: Fix this error by deriving the appropriate traits for DBState
            assert_eq!(read_result, state);
        }
    }
}