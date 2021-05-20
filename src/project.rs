//! Interfaces for accessing and managing sprints

// Third party


// Ours
use crate::{Jira, Result};

#[derive(Debug)]
pub struct Project {
    jira: Jira,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
    pub description: Option<String>,
    pub name: String,
    pub id: String,
    #[serde(rename = "self")]
    pub self_link: String,
}

impl Project {
    pub fn new(jira: &Jira) -> Project {
        Project { jira: jira.clone() }
    }

    /// get all components in a project
    /// https://docs.atlassian.com/jira-software/REST/latest/#agile/1.0/board/{boardId}/sprint-getAllSprints
    pub fn components(&self, project_or_key: &str) -> Result<Vec<Component>> {
        let path = format!("/project/{}/components", project_or_key);

        self.jira
            .get::<Vec<Component>>("api", path.as_ref())
    }

    // pub fn statuses(&self, project_or_key: &str) -> Result<Vec<Status>> {
    //     let mut path = format!("/project/{}/statuses", project_or_key);
    //
    //     self.jira
    //         .get::<Vec<Status>>("api", path.as_ref())
    // }
}
