use actix_web::{web::{self, Data}, App, HttpServer};
use handlers::{add_new_task, add_new_tasks, delete_task, get_all_tasks, get_task, get_tasks_status, update_task_status};
use rusqlite::Connection;
mod routes;
mod models;  
   
mod handlers; 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
     HttpServer::new(move || {
        App::new()
        .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}