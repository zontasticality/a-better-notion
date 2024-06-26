use actix_web::{get, web, Responder, Result};
use common::backend::*;
use sea_orm::entity::prelude::*;

use crate::database::task;
/* // Define a new type that wraps DbErr
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
} */

// Implement ResponseError for the new type
/* impl ResponseError for MyDbErr {
    fn error_response(&self) -> HttpResponse {
        // Customize the HTTP response based on the error
        HttpResponse::InternalServerError().json("Internal server error")
    }
} */

/// get /task endpoint for retrieving a single TaskShort
#[get("/task")]
async fn get_task_request(
    data: web::Data<DatabaseConnection>,
    req: web::Json<ReadTaskShortRequest>,
) -> Result<impl Responder> {
    let db = data;
    let task = task::Entity::find_by_id(req.task_id)
        .one(db.as_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
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
        None => Err(actix_web::error::ErrorNotFound("task not found by ID")),
    }
}

/// get /tasks endpoint for retrieving some number of TaskShorts
#[get("/tasks")]
async fn get_tasks_request(
    data: web::Data<DatabaseConnection>,
    req: web::Json<ReadTasksShortRequest>,
) -> Result<impl Responder> {
    let mut res: ReadTasksShortResponse = Vec::new();

    for taskreq in req.iter() {
        let task = task::Entity::find_by_id(taskreq.task_id)
            .one(data.as_ref())
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        match task {
            Some(model) => res.push(Ok(ReadTaskShortResponse {
                task_id: model.id,
                name: model.title,
                completed: model.completed,
                props: Vec::new(),
                deps: Vec::new(),
                scripts: Vec::new(),
                last_edited: model.last_edited,
            })),
            None => res.push(Err("task not found by ID".to_string())),
        }
    }

    Ok(web::Json(res))
}

/// get /filter endpoint for retrieving some number of TaskShorts
#[allow(unused_variables)]
#[get("/filter")]
async fn get_filter_request(
    data: web::Data<DatabaseConnection>,
    req: web::Json<FilterRequest>,
) -> Result<impl Responder> {
    //TODO: construct filter

    let tasks: Vec<task::Model> = task::Entity::find()
        .all(data.as_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(web::Json(
        tasks.iter().map(|a| a.id).collect::<FilterResponse>(),
    ))
}
