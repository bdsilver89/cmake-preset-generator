use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowPreset {
    name: String,

    #[serde(
        rename(serialize = "displayName", deserialize = "displayName"),
        skip_serializing_if = "Option::is_none"
    )]
    display_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    steps: Vec<WorkflowStep>,
}

impl WorkflowPreset {
    pub fn new(name: &str) -> Self {
        WorkflowPreset {
            name: name.into(),
            display_name: None,
            description: None,
            steps: Vec::<WorkflowStep>::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn display_name(&self) -> &Option<String> {
        &self.display_name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn steps(&self) -> &Vec<WorkflowStep> {
        &self.steps
    }

    pub fn set_name(&mut self, name: &String) {
        self.name = name.into();
    }

    pub fn set_display_name(&mut self, display_name: &str) {
        self.display_name = Some(display_name.into());
    }

    pub fn clear_display_name(&mut self) {
        self.display_name = None;
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = Some(description.into());
    }

    pub fn clear_description(&mut self) {
        self.description = None
    }

    pub fn add_step(&mut self, type_name: &str, name: &str) {
        self.steps.push(WorkflowStep::new(type_name, name));
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowStep {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    pub name: String,
}

impl WorkflowStep {
    pub fn new(type_name: &str, name: &str) -> Self {
        WorkflowStep {
            type_name: type_name.into(),
            name: name.into(),
        }
    }
}
