use std::{collections::HashMap, fmt::Display};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail {epic_id: u32},
    NavigateToStoryDetail { story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus {epic_id: u32},
    DeleteEpic {epic_id: u32},
    CreateStory {epic_id: u32},
    UpdateStoryStatus { story_id: u32 },
    DeleteStory {epic_id: u32, story_id: u32},
    Exit,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum Status {
    // TODO: add fields (make sure the fields are public)
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Epic {
    // TODO: add fields (make sure the fields are public)
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        // todo!() by default the status should be set to open and the stories should be an empty vector
        Self {
            name,
            description,
            status: Status::Open,
            stories: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Story {
    // TODO: add fields (make sure the fields are public)
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        // todo!() // by default the status should be set to open
        Self {
            name,
            description,
            status: Status::Open,        
        }    
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
} //added 'use std::fm::Display to allow fields within DBState to be Cloned (Clone needs Display trait)
    // Also derived Clone trait to Epic, Story and DBState data types