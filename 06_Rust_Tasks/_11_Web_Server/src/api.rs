use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::handler::save_drivers;
use crate::model::Driver;

pub type SharedState = Arc<RwLock<Vec<Driver>>>;

// GET /drivers
pub async fn get_drivers(
    State(state): State<SharedState>,
) -> Json<Vec<Driver>> {
    println!("get_drivers handled by thread: {:?}", std::thread::current().id());
    
    let drivers = state.read().await;
    Json(drivers.clone())
}

// GET /drivers/:id
pub async fn get_driver(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Driver>, StatusCode> {
    println!("get_driver ({}) handled by thread: {:?}", id, std::thread::current().id());

    let drivers = state.read().await;
    drivers
        .iter()
        .find(|d| d.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// POST /drivers
pub async fn add_driver(
    State(state): State<SharedState>,
    Json(mut driver): Json<Driver>,
) -> (StatusCode, Json<Driver>) {
    println!("add_driver handled by thread: {:?}", std::thread::current().id());

    let mut drivers = state.write().await;
    let new_id = format!("DRV-{:03}", drivers.len() + 1);
    driver.id = new_id;
    drivers.push(driver.clone());
    save_drivers(&drivers);
    (StatusCode::CREATED, Json(driver))
}

// PUT /drivers/:id
pub async fn update_driver(
    Path(id): Path<String>,
    State(state): State<SharedState>,
    Json(updated): Json<Driver>,
) -> Result<Json<Driver>, StatusCode> {
    println!("update_driver ({}) handled by thread: {:?}", id, std::thread::current().id());

    let mut drivers = state.write().await;
    if let Some(index) = drivers.iter().position(|d| d.id == id) {
        drivers[index].name = updated.name;
        drivers[index].team = updated.team;
        drivers[index].points = updated.points;

        let updated_driver = drivers[index].clone();
        save_drivers(&drivers);
        return Ok(Json(updated_driver));
    }
    Err(StatusCode::NOT_FOUND)
}

// DELETE /drivers/:id
pub async fn delete_driver(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> StatusCode {
    println!("delete_driver ({}) handled by thread: {:?}", id, std::thread::current().id());

    let mut drivers = state.write().await;
    let initial_len = drivers.len();
    drivers.retain(|d| d.id != id);

    if drivers.len() < initial_len {
        save_drivers(&drivers);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}