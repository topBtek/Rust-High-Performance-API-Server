use actix_web::{test, web, App};
use rust_high_performance_api_server::{
    create_app,
    models::{CreateTaskRequest, Task},
    state::AppState,
};
use serde_json::json;

#[actix_web::test]
async fn test_health_check() {
    let app_state = AppState::new();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .route("/health", web::get().to(rust_high_performance_api_server::handlers::health_check)),
    )
    .await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_create_task() {
    let app_state = AppState::new();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/api/v1/tasks", web::post().to(rust_high_performance_api_server::handlers::create_task)),
    )
    .await;

    let create_req = CreateTaskRequest {
        title: "Test Task".to_string(),
        description: Some("Test Description".to_string()),
    };

    let req = test::TestRequest::post()
        .uri("/api/v1/tasks")
        .set_json(&create_req)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let task: Task = test::read_body_json(resp).await;
    assert_eq!(task.title, "Test Task");
    assert_eq!(task.description, Some("Test Description".to_string()));
    assert!(!task.completed);
}

#[actix_web::test]
async fn test_get_tasks() {
    let app_state = AppState::new();
    
    // Create a test task
    let task = Task::new("Test Task".to_string(), None);
    let task_id = task.id;
    app_state.tasks.insert(task_id, task);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .route("/api/v1/tasks", web::get().to(rust_high_performance_api_server::handlers::get_tasks)),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/v1/tasks").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let tasks: Vec<Task> = test::read_body_json(resp).await;
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].title, "Test Task");
}

#[actix_web::test]
async fn test_get_task_by_id() {
    let app_state = AppState::new();
    
    let task = Task::new("Test Task".to_string(), None);
    let task_id = task.id;
    app_state.tasks.insert(task_id, task);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .route("/api/v1/tasks/{id}", web::get().to(rust_high_performance_api_server::handlers::get_task)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/tasks/{}", task_id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let task: Task = test::read_body_json(resp).await;
    assert_eq!(task.id, task_id);
}

#[actix_web::test]
async fn test_update_task() {
    let app_state = AppState::new();
    
    let task = Task::new("Original Title".to_string(), None);
    let task_id = task.id;
    app_state.tasks.insert(task_id, task);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .route("/api/v1/tasks/{id}", web::put().to(rust_high_performance_api_server::handlers::update_task)),
    )
    .await;

    let update_req = json!({
        "title": "Updated Title",
        "completed": true
    });

    let req = test::TestRequest::put()
        .uri(&format!("/api/v1/tasks/{}", task_id))
        .set_json(&update_req)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let task: Task = test::read_body_json(resp).await;
    assert_eq!(task.title, "Updated Title");
    assert!(task.completed);
}

#[actix_web::test]
async fn test_delete_task() {
    let app_state = AppState::new();
    
    let task = Task::new("Test Task".to_string(), None);
    let task_id = task.id;
    app_state.tasks.insert(task_id, task);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .route("/api/v1/tasks/{id}", web::delete().to(rust_high_performance_api_server::handlers::delete_task)),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri(&format!("/api/v1/tasks/{}", task_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);
}

#[actix_web::test]
async fn test_create_task_validation() {
    let app_state = AppState::new();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .route("/api/v1/tasks", web::post().to(rust_high_performance_api_server::handlers::create_task)),
    )
    .await;

    let create_req = CreateTaskRequest {
        title: "   ".to_string(), // Empty after trim
        description: None,
    };

    let req = test::TestRequest::post()
        .uri("/api/v1/tasks")
        .set_json(&create_req)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
