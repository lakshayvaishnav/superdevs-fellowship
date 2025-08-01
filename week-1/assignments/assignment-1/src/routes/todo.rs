use std::sync::{ Arc, Mutex };
use traity_trait::todo_app;
use actix_web::web::{ Data, Json };
use actix_web::{ HttpResponse, Responder, get, post };
use serde::{ Serialize, Deserialize };
use crate::middleware::UserId;
use crate::db::Db;

#[todo_app]
#[derive(Serialize, Deserialize)]
struct CreateTodoResponse {
    message: String,
}

#[todo_app]
#[derive(Serialize, Deserialize)]
struct CreateTodoRequest {
    pub text: String,
}

#[todo_app]
#[derive(Serialize, Deserialize)]
struct GetTodosResponse {
    user_todos: Vec<String>,
}

#[post("/todo")]
pub async fn create_todo(
    user_id: UserId,
    db: Data<Arc<Mutex<Db>>>,
    body: Json<CreateTodoRequest>
) -> impl Responder {
    let mut db = db.lock().unwrap();
    db.create_todo(user_id.0, body.text.clone());
    HttpResponse::Ok().json(CreateTodoResponse {
        message: "Todo created".to_string(),
    })
}

#[get("/todos")]
pub async fn get_todos(user_id: UserId, db: Data<Arc<Mutex<Db>>>) -> impl Responder {
    let db = db.lock().unwrap();
    let todos = db.get_todos(user_id.0);
    HttpResponse::Ok().json(GetTodosResponse {
        user_todos: todos,
    })
}
