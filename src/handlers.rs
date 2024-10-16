use std::{path, sync::{Arc, Mutex}};
use actix_web::{delete, dev::Path, get, post, put, rt::task, web, HttpResponse, Responder};
use crate::models::{NewTask, Task, TaskResponse, UpdateTask};
use rusqlite::{params, Connection, Result};


#[get("/tasks")]
pub async fn get_all_tasks(conn:web::Data<Arc<Mutex<Connection>>>)->HttpResponse{
   let conn=conn.lock().unwrap();
    let mut  stmt=conn.prepare("SELECT * FROM Task").unwrap();
    let tasks=stmt.query_map([], |row|{
        Ok(Task{
            id:row.get(0)?,
            user_id:row.get(1)?,
            title:row.get(2)?,
            description:row.get(3)?,
            priority:row.get(4)?,
            status:row.get(5)?
        })
    }).unwrap();
    let tasks: Vec<Task> = tasks.filter_map(Result::ok).collect();

    HttpResponse::Ok().json(tasks)
   
}
#[get("tasks/{id}")]
pub async  fn get_task(path:web::Path<u32>,conn:web::Data<Arc<Mutex<Connection>>>)->HttpResponse{
    let conn=conn.lock().unwrap();
    let id=path.into_inner();
    let mut stmt=conn.prepare("SELECT * FROM Task WHERE id = ?1").unwrap();
    let task=stmt.query_map([id], |row|{
        Ok(TaskResponse{
            title:row.get(2)?,
            description:row.get(3)?,
           status:row.get(5)?

            
        
        })
    }).unwrap();
    let task:Vec<TaskResponse>=task.filter_map(Result::ok).collect();
   
    

    HttpResponse::Ok().json(task)

}
#[get("tasks/s/{status}")]
pub async fn get_tasks_status(status:web::Path<String>,conn:web::Data<Arc<Mutex<Connection>>>)->HttpResponse{
    let conn=conn.lock().unwrap();
    let status=status.into_inner();
    let mut stmt=conn.prepare("SELECT * FROM Task WHERE status LIKE '%' || ?1 || '%'").unwrap();
    let tasks=stmt.query_map(params![status.to_string()], |row|{
        Ok(
        TaskResponse{
            title:row.get(2)?,
            description:row.get(3)?,
            status:row.get(5)?
        }

        )
    }).unwrap();
    
    let tasks:Vec<TaskResponse>=tasks.filter_map(Result::ok).collect();
    HttpResponse::Ok().json(tasks)

}
#[post("/task")]
pub async fn add_new_task(task:web::Json<NewTask>,conn:web::Data<Arc<Mutex<Connection>>>)->HttpResponse{
    let conn=conn.lock().unwrap();
    let result=conn.execute(
        "INSERT INTO Task (user_id,title,description,priority,status) 
         VALUES (?=1,?2,?3,?4,?5)
        "
        
        
        , params![&task.user_id,&task.title,&task.description,&task.priority,&task.status]);
        match result {
            Ok(_)=>HttpResponse::Created().json("Task succesfully added"),
            Err(e)=>{
                eprintln!("failed to insert task :{}",e);
                HttpResponse::InternalServerError().json("failed")
            }
            
        }
}
#[post("/tasks")]
pub async fn add_new_tasks(tasks:web::Json<Vec<NewTask>>,conn:web::Data<Arc<Mutex<Connection>>>)->HttpResponse{
    let mut conn=conn.lock().unwrap();
    let tx=conn.transaction().unwrap();
    for task in tasks.into_inner(){
        let result=tx.execute(
            "INSERT INTO Task (user_id, title, description, priority, status) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &task.user_id,
                &task.title,
                &task.description,
                &task.priority,
                &task.status,
            ]);
            if result.is_err(){
                tx.rollback().unwrap();
                return HttpResponse::InternalServerError().json("failed to add tasks");
            }

    }
    tx.commit().unwrap();
    HttpResponse::Ok().json("tasks added successfully")
}
#[delete("/tasks/{id}")]
pub async  fn delete_task(path:web::Path<i32>,conn:web::Data<Arc<Mutex<Connection>>>)->HttpResponse{
    let conn=conn.lock().unwrap();
    let task_id=path.into_inner();
    let res=conn.execute("DELETE FROM Task WHERE id =?1", params![task_id]);

    match res{
        Ok(affected_rows)=>{
            if affected_rows==0 {
                HttpResponse::NotFound().json("task not found")
             }else {
                 HttpResponse::Ok().json("task sucesfullly deleted")
             }
        }
        Err(e)=>{
            eprint!("failed to delete task :{}",e);
            HttpResponse::InternalServerError().json("failed to delete task")
        }
    }
   

}

#[put("/task/update/{id}")]
async fn update_task_status(id: web::Path<i32>, updated_task: web::Json<UpdateTask>,conn:web::Data<Arc<Mutex<Connection>>>) -> HttpResponse {
    let id = id.into_inner();
    let conn = conn.lock().unwrap();
    let res = conn.execute("UPDATE Task SET status = ?1 WHERE id = ?2", params![updated_task.status, id]);
    match res {
        Ok(affected_rows) => {
            if affected_rows == 0 {
                HttpResponse::NotFound().json("Task not found")
            } else {
                HttpResponse::Ok().json("Status updated successfully")
            }
        }
        Err(e) => {
            eprintln!("Failed to update status: {}", e);
            HttpResponse::InternalServerError().json("Failed to update status")
        }
    }
}


