use actix_web::web;
use crate::handlers::{self, *};


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::get_all_tasks);
    cfg.service(get_task);
    cfg.service(update_task_status);
    cfg.service(delete_task);
    cfg.service(add_new_tasks);
    cfg.service(handlers::add_new_task);
    cfg.service(handlers::get_tasks_status);

    
   
}