use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Driver {
    //  Writing of the id field will depend on the database and also for now ignore it  on creating time
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub team: String,
    pub points: u32,
}
