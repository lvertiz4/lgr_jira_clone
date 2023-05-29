use anyhow::{anyhow, Result, Context, Ok};
//'anyhow' library provides a trait object called 'Error', to help handle errors.
//A trait object points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtime. 
// The 'Error' type, a wrapper around a dynamic error type.Error works a lot like Box<dyn std::error::Error>
//Error associated function 'downcast' Attempt to downcast the error object to a concrete type, from a dynamic trait object to a concrete type of a known size
use std::rc::Rc;

use crate::{ui::{Page, HomePage, EpicDetail, StoryDetail, Prompts}, db::JiraDatabase, models::Action};

pub struct Navigator {
    pages: Vec<Box<dyn Page>>, //Page is a trait object, so it is wrapped in a Box pointer to provie a known size (the pointer) when the object will be dynamically dispatched at runtime, dyn is a prefix of a trait object's type
    prompts: Prompts,
    db: Rc<Database>
}

impl Navigator {
    pub fn new(db: Rc<Database>) -> Self {

    }

    pub fn get_current_page(&self) -> Option<&Box<dyn Page>> { //Function will always return the last element of the vector
    
    }

    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::NavigateToEpicDetail {epic_id} => { //create a new EpicDetail instance and add it to the pages vector

            }
            Action::NavigateToStoryDetail {epic_id, story_id} => {///create a new StoryDetail instance and add it to the pages vector

            }
            Action::NavigateToPreviousPage => { //remove the last page from the pages vector
            }
            Action::CreateEpic => {//prompt the user to create a new epic and presist it in the database

            }
            Action::UpdateEpicStatus {epic_id} => {//prompt the user to update status and persis it in the database

            }
            Action::DeleteEpic {epic_id} => {//prompt the user to to delete the epic and persist it in the database
            
            }
            Action::CreateStory {epic_id} => {//prompt the user to create a new story and persist it in the database
            
            }
            Action::UpdateStoryStatus {story_id} => {//prompt the user to update status and persist it in the database
            
            }
            Action::DeleteStory {epic_id, story_id} => {//prompt the user to delete the story and persist it in the database
            
            }
            Action::Exit => {//remove all pages from the page vector
            
            },
        }
        Ok(())
    }
    //Private functions used for testing
    fn get_page_count(&self) -> usize {
        self.pages.len()
    }

    fn set_prompts(&mut self, prompts: Prompts) {
        self.prompts = prompts;
    }
}

#[cfg(test)]//enables conditional compilation; 
//1annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build
mod tests {
    use crate::{db::test_utils::MockDB, models::{Epic, Status, Story}};
    use super::*; //`super` keyword refers to the parent scope (outside the `tests` module)

    #[test]
    fn should_start_on_home_page() {
        let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
        let nav = Navigator::new(db);

        assert_eq!(nav.get_page_count(), 1);

        let current_page = nav.get_current_page().unwrap();
        let home_page = current_page.as_any().downcast_ref::<HomePage>(); //The indirection through `as_any` is because using `downcast_ref`
        // on `Box<A>` *directly* only lets us downcast back to `&A` again.
        assert_eq!(home_page.is_some(), true);//reminder: is_some() returns true if Option contains 'some' value
    }

    fn handle_action_should_navigate_pages() {
        let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
        let mut nav = Navigator::new(db);

        nav.handle_action(Action::NavigateToEpicDetail { epic_id: 1}).unwrap();
        assert_eq!(nav.get_page_count(), 2);

        let current_page = nav.get_current_page().unwrap();//remember unwrap takes value from Option is it is Some(
        let epic_detail_page = current_page.as_any().downcast_ref::<EpicDetail>();//dynamic trait object to a concrete type of a known size, which is, here, an EpicDetail struct; Downcast this error object by reference since get_current_page takes a reference to self (i.e. Navigator struct)
        assert_eq!(epic_detail_page.is_some(), true);

        nav.handle_action(Action::NavigateToStoryDetail {epic_id: 1, story_id: 2}).unwrap();//remember: handle_action() returns a Result, unwrap gets the value if Result is Ok();
        assert_eq!(nav.get_page_count(), 3);

        let current_page = nav.get_current_page().unwrap();
        let story_detail_page = current_page.as_any().downcast_ref::<StoryDetail>();
        assert_eq!(story_detail_page.is_some(), true);

        nav.handle_action(Action::NavigateToPreviousPage);
        assert_eq!(nav.get_page_count(), 2);
        
        let current_page = nav.get_current_page().unwrap();
        let epic_detail_page = current_page.as_any().downcast_ref::<EpicDetail>();
        assert_eq!(epic_detail_page.is_some(), true);

        nav.handle_action(Action::NavigateToPreviousPage).unwrap();
        assert_eq!(nav.get_page_count(), 1);

        let current_page = nav.get_current_page().unwrap();
        let home_page = current_page.as_any().downcast_ref::<HomePage>();
        assert_eq!(home_page.is_some(), true);

        nav.handle_action(Action::NavigateToPreviousPage).unwrap();
        
        assert_eq!(nav.get_page_count(),0);
        
        nav.handle_action(Action::NavigateToPreviousPage).unwrap();
        assert_eq!(nav.get_page_count(),0);
    }

    #[test]
    fn handle_action_should_clear_pages_on_exit() {
        let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
        let mut nav = Navigator::new(db);
        nav.handle_action(Action::NavigateToEpicDetail {epic_id: 1}).unwrap();
        nav.handle_action(Action::NavigateToStoryDetail {epic_id: 1, story_id: 2}).unwrap();
        nav.handle_action(Action::Exit).unwrap();
        assert_eq!(nav.get_page_count(), 0);
    }

    #[test]
    fn handle_action_should_handle_create_epic() {
        let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
        let mut nav = Navigator::new(Rc::clone(&db));
        let mut prompts = Prompts::new();

        prompts.create_epic = Box::new(|| Epic::new("name".to_owned(), "description".to_owned()));
        nav.set_prompts(prompts);
        nav.handle_action(Action::CreateEpic).unwrap();
        
        let db_state = db.read_db().unwrap();
        assert_eq!(db_state.epics.len(), 1);

        let epic = db_state.epics.into_inter().next().unwrap().1;
        assert_eq!(epic.name, "name".to_owned());
        assert_eq!(epic.description, "description".to_owned());
    }

    #[test]
    fn handle_action_should_handle_update_epic() {
        let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
        let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
        let mut nav = Navigator.new(Rc::clone(&db));
        let mut prompts = Prompts::new();

        prompts.update_status = Box::new(|| Some(Status::InProgress));
        nav.set_prompts(prompts);
        nav.handle_action(Action::UpdateEpicStatus {epid_id}.unwrap());

        let db_state = db.read_db().unwrap();
        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::InProgress);
    }

    #[test]
    fn handle_action_should_handle_delete_epic() {
        let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
        let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
        let mut nav = Navigator::new(Rc::clone(&db));
        let mut prompts = Prompts::new();

        prompts.delete_epic = Box::new(|| true);
        nav.set_prompts(prompts);
        nav.handle_action(Action::DeleteEpic {epic_id}).unwrap();
        let db_state = db.read_db().unwrap();
        assert_eq!(db_state.epics.len(), 0);
    }

    #[test]
    fn handle_action_should_handle_create_story() {
        let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
        let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
        let mut nav = Navigator::new(Rc::clone(&db));
        let mut prompts = Prompts::new();

        prompts.create_story = Box::new(|| Story::new("name".to_owned(), "description".to_owned()));
        nav.set_prompts(prompts);
        nav.handle_action(Action::CreateStory {epic_id}).unwrap();
        let db_state = db.read_db().unwrap();
        assert_eq!(db_state.stories.len(), 1);

        let story = db_state.stories.inter_iter().next().unwrap().1;
        assert_eq!(story.name, "".to_owned());
        assert_eq!(story.description, "".to_owned());
    }

    #[test]
    fn handle_action_should_handle_update_story() {
        let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
        let epic_id = db.create_epic(Epic::new(""to_owned(), "".to_owned())).unwrap();
        let story_id = db.create_story(Story::new(""to_owned(), "".to_owned()), epic_id).unwrap();
        let mut nav = Navigator::new(Rc::clone(&db));
        let mut prompts = Prompts::new();

        prompts.update_status = Box::new(|| Some(Status::InProgress));
        nav.set_prompts(prompts);
        nav.handle_action(Action::UpdateStoryStatus {story_id}).unwrap();
        let db_state = db.read_db().unwrap();
        assert_eq!(db_state.stories.get(&story_id).unwrap().status, Status::InProgress);
    }

    #[test]
    fn handle_action_should_delete_story() {
        let db = Rc::new(JiraDatabase {database: Box::new(MockDB::new())});
        let epic_id = db.create_epic(Epic::new("".to_owned(), "".to_owned())).unwrap();
        let story_id = db.create_story(Story::new("".to_owned(), "".to_owned(), epic_id).unwrap());
        let mut nav = Navigator::new(Rc::clone(&db));
        let mut prompts = Prompts::new();

        prompts.delete_story = Box::new(|| true);
        nav.set_prompts(prompts);
        nav.handle_action(Action::DeleteStory {epic_id, story_id}).unwrap();
        let db_state = db.read_db().unwrap();
        assert_eq!(db_state.stories.len(), 0);
    }
}