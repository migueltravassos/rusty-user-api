use axum::{
    extract::{Path, State},
    routing::{get, post}, 
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// O "Banco de Dados" na RAM
struct AppState {
    users: Vec<User>,
    next_id: u64,
}

#[derive(Serialize, Clone, Debug)]
struct User {
    user_id: u64,
    username: String,
    message: String,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
}

#[tokio::main]
async fn main() {
    // Inicializar o estado com ID a começar em 1
    let shared_state = Arc::new(Mutex::new(AppState {
        users: Vec::new(),
        next_id: 1,
    }));

    // Definir rotas
    let app = Router::new()
        .route("/", get(say_hello))
        .route("/users", post(create_user).get(get_all_users)) // GET e POST no mesmo URL
        .route("/users/:id", get(get_single_user).delete(delete_user)) 
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on port 3000");

    axum::serve(listener, app).await.unwrap();
}

async fn say_hello() -> &'static str {
    "Welcome to Rusty API! 🦀"
}

// Handler para CRIAR utilizador (Thread-Safe)
async fn create_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<User> {
    let mut guard = state.lock().unwrap(); // Bloqueia o Mutex para escrever

    let id = guard.next_id;
    guard.next_id += 1;

    let new_user = User {
        user_id: id,
        username: payload.username,
        message: format!("User {} created successfully!", id),
    };

    guard.users.push(new_user.clone());
    Json(new_user)
}

// Handler para LER todos
async fn get_all_users(State(state): State<Arc<Mutex<AppState>>>) -> Json<Vec<User>> {
    let guard = state.lock().unwrap();
    Json(guard.users.clone())
}

// Handler para LER um (pelo ID)
async fn get_single_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<u64>,
) -> Json<Option<User>> {
    let guard = state.lock().unwrap();
    let user = guard.users.iter().find(|u| u.user_id == id).cloned();
    Json(user)
}

// Handler para APAGAR
async fn delete_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<u64>,
) -> Json<String> {
    let mut guard = state.lock().unwrap();
    let initial_len = guard.users.len();
    
    guard.users.retain(|user| user.user_id != id);

    if guard.users.len() < initial_len {
        Json(format!("User {} deleted.", id))
    } else {
        Json(format!("User {} not found.", id))
    }
}