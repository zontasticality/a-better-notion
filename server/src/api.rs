use actix_web::{
    get,
    http::{Error, StatusCode},
    web, HttpResponse, Responder, ResponseError, Result,
};
use common::backend::*;
use sea_orm::{entity::prelude::*, Database, DbErr};
use std::fmt;

use crate::database::task;
// Define a new type that wraps DbErr
pub struct MyDbErr(DbErr);

impl fmt::Debug for MyDbErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for MyDbErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

// Implement ResponseError for the new type
impl ResponseError for MyDbErr {
    fn error_response(&self) -> HttpResponse {
        // Customize the HTTP response based on the error
        HttpResponse::InternalServerError().json("Internal server error")
    }
}

// get /task endpoint for retrieving a single TaskShort
#[get("/task")]
async fn get_task_request(req: web::Json<ReadTaskShortRequest>) -> Result<impl Responder> {
    println!("requesting task id {}", req.task_id);
    let db: DatabaseConnection =
        Database::connect("postgres://abn:abn@localhost:5432/abn?currentSchema=task")
            .await
            .map_err(|_| {
                actix_web::error::ErrorInternalServerError(
                    "failed to connect to the database inside server",
                )
            })?; //TODO 26mar24 Mrknox: this is a hardcoded connection string, we should probably move this to a config file or something

    let task = task::Entity::find_by_id(req.task_id)
        .one(&db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(MyDbErr(e)))?;
    match task {
        Some(model) => Ok(web::Json(ReadTaskShortResponse {
            task_id: model.id,
            name: model.title,
            completed: model.completed,
            props: Vec::new(),   //TODO 26mar24 Mrknox: implement properties
            deps: Vec::new(),    //TODO 26mar24 Mrknox: implement dependencies
            scripts: Vec::new(), //TODO 26mar24 Mrknox: implement scripts
            last_edited: model.last_edited,
        })),
        None => {
            return Err(actix_web::error::ErrorNotFound("task not found by ID"));
        }
    }
}
