use crate::components::{CreateTodo, SqliteStorage, Storage, Todo};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::Utc;
use std::sync::Arc;

pub fn create_router(storage: Arc<SqliteStorage>) -> Router {
    // 创建 SQLite 存储
    // let storage = Arc::new(SqliteStorage::new("todos.db").expect("Failed to create storage"));

    Router::new()
        .route("/", get(root))
        .route("/todos", get(list_todos).post(add_todo))
        .route(
            "/todos/:id",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        .with_state(storage)

    // // 构建我们的应用，只有一个路由
    // let app = Router::new()
    //     .route("/", get(root))
    //     .route("/todos", get(list_todos).post(add_todo))
    //     .route(
    //         "/todos/:id",
    //         get(get_todo).put(update_todo).delete(delete_todo),
    //     )
    //     .with_state(storage);

    // println!("Server running on http://localhost:3000");
    // // 运行我们的应用
    // // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

// 基本的处理函数
async fn root() -> &'static str {
    "Hello, World!"
}

async fn list_todos(
    State(storage): State<Arc<SqliteStorage>>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    match storage.list() {
        Ok(todos) => Ok(Json(todos)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn add_todo(
    State(storage): State<Arc<SqliteStorage>>,
    Json(todo): Json<CreateTodo>,
) -> impl IntoResponse {
    let new_todo = Todo {
        id: 0, // This will be ignored by SQLite and auto-incremented
        title: todo.title,
        description: todo.description,
        completed: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    match storage.add(&new_todo) {
        Ok(_) => (StatusCode::CREATED, "Todo created successfully").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create todo").into_response(),
    }
}

async fn get_todo(
    State(storage): State<Arc<SqliteStorage>>,
    Path(id): Path<u64>,
) -> Result<Json<Todo>, StatusCode> {
    match storage.get(id) {
        Ok(Some(todo)) => Ok(Json(todo)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_todo(
    State(storage): State<Arc<SqliteStorage>>,
    Path(id): Path<u64>,
    Json(todo): Json<Todo>,
) -> Result<StatusCode, StatusCode> {
    if id != todo.id {
        return Err(StatusCode::BAD_REQUEST);
    }
    match storage.update(&todo) {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_todo(
    State(storage): State<Arc<SqliteStorage>>,
    Path(id): Path<u64>,
) -> Result<StatusCode, StatusCode> {
    match storage.delete(id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
