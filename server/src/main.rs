//! Server-Side API crate

#![warn(rustdoc::private_doc_tests)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
mod api;
mod database;
use actix_web::{web::Data, App, HttpServer};
use api::*;
use sea_orm::{Database, DatabaseConnection};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");

    let db = Database::connect("postgres://abn:abn@localhost:5432/abn?currentSchema=task")
        .await
        .unwrap();
    let db_data: Data<DatabaseConnection> = Data::new(db);
    HttpServer::new(move || {
        let db_data = db_data.clone();
        App::new()
            .app_data(db_data)
            .service(get_task_request)
            .service(get_tasks_request)
            .service(get_filter_request)
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
#[cfg(test)]
mod tests {
    use super::*;
    use common::{backend::*, Filter};

    #[test]
    fn test_main() {
        std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_millis(500));
            std::process::exit(0)
        });
        main().unwrap();
    }

    #[actix_web::test]
    async fn task_request() {
        use actix_web::test;
        use sea_orm::MockDatabase;

        let db = MockDatabase::new(sea_orm::DatabaseBackend::Postgres);
        let db_conn = db
            .append_query_results([vec![database::task::Model {
                id: 1,
                title: "test".to_string(),
                completed: false,
                last_edited: chrono::NaiveDateTime::default(),
            }]])
            .into_connection();
        let db_data: Data<DatabaseConnection> = Data::new(db_conn);
        let app = test::init_service(App::new().app_data(db_data).service(get_task_request)).await;
        let req = test::TestRequest::default()
            .set_json(ReadTaskShortRequest { task_id: 1 })
            .uri("/task")
            .to_request();
        let resp: ReadTaskShortResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.task_id, 1);
    }

    #[actix_web::test]
    async fn task_requests() {
        use actix_web::test;
        use sea_orm::MockDatabase;

        let db = MockDatabase::new(sea_orm::DatabaseBackend::Postgres);
        let db_conn = db
            .append_query_results([
                vec![database::task::Model {
                    id: 1,
                    title: "test".to_string(),
                    completed: false,
                    last_edited: chrono::NaiveDateTime::default(),
                }],
                vec![database::task::Model {
                    id: 2,
                    title: "test2".to_string(),
                    completed: false,
                    last_edited: chrono::NaiveDateTime::default(),
                }],
                vec![],
            ])
            .into_connection();
        let db_data: Data<DatabaseConnection> = Data::new(db_conn);
        let app = test::init_service(App::new().app_data(db_data).service(get_tasks_request)).await;
        let req = test::TestRequest::default()
            .set_json(vec![
                ReadTaskShortRequest { task_id: 1 },
                ReadTaskShortRequest { task_id: 2 },
                ReadTaskShortRequest { task_id: 3 },
            ])
            .uri("/tasks")
            .to_request();
        let resp: ReadTasksShortResponse = test::call_and_read_body_json(&app, req).await;

        assert!(resp[0].as_ref().is_ok_and(|a| a.task_id == 1));
        assert!(resp[1].as_ref().is_ok_and(|a| a.task_id == 2));
        assert!(resp[2].is_err());
    }

    #[actix_web::test]
    async fn filter_request() {
        use actix_web::test;
        use sea_orm::MockDatabase;

        let db = MockDatabase::new(sea_orm::DatabaseBackend::Postgres);
        let db_conn = db
            .append_query_results([vec![
                database::task::Model {
                    id: 1,
                    title: "heyo".to_owned(),
                    completed: true,
                    last_edited: chrono::NaiveDateTime::default(),
                },
                database::task::Model {
                    id: 2,
                    title: "heyo".to_owned(),
                    completed: true,
                    last_edited: chrono::NaiveDateTime::default(),
                },
            ]])
            .into_connection();
        let db_data: Data<DatabaseConnection> = Data::new(db_conn);
        let app =
            test::init_service(App::new().app_data(db_data).service(get_filter_request)).await;
        let req = test::TestRequest::default()
            .set_json(FilterRequest {
                filter: Filter::None,
            })
            .uri("/filter")
            .to_request();
        let resp: FilterResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp[0], 1);
        assert_eq!(resp[1], 2);
    }
}
