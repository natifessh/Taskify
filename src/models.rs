use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Deserialize,Serialize)]
pub struct TaskResponse{
    pub title:String,
    pub description:String,
    pub status:String
}


#[derive(Deserialize,Serialize)]
pub struct NewTask {
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTask {
    pub status: String,
}


