use actix_web::{delete, get, post, put, web, HttpResponse};
use std::{ops::Deref, sync::RwLock};
//use std::env;

use crate::errors::{AppResponse, HttpAppError};
use crate::person::Person;

pub struct AppState {
    pub person_collection: RwLock<Vec<Person>>,
    pub greeting_text: String, // New field for greeting text
}

#[get("/")]
async fn landing_page(data: web::Data<AppState>) -> AppResponse {
    use chrono::Utc; // Add chrono to the imports

    let current_time = Utc::now().to_rfc3339(); // Get the current UTC time in ISO 8601 format
    let response_body = format!("{} Current UTC time: {}", data.greeting_text, current_time); // Combine greeting text with the time

    Ok(HttpResponse::Ok().body(response_body))
}

// Catch-all handler for unknown paths
pub async fn not_found_handler() -> AppResponse {
    Ok(HttpResponse::NotFound().body("Oops! The page you are looking for does not exist."))
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/api/persons")]
async fn persons(data: web::Data<AppState>) -> AppResponse {
    let persons = data.person_collection.read()?;
    Ok(HttpResponse::Ok().json(persons.deref()))
}

#[get("/api/person/{id}")]
async fn single_person(path: web::Path<u32>, data: web::Data<AppState>) -> AppResponse {
    let id = path.into_inner();
    let persons_guard = data.person_collection.read()?;
    let filtered = persons_guard.iter().find(|t| t.id == id);
    match filtered {
        Some(filtered) => Ok(HttpResponse::Ok().json(filtered)),
        None => Err(HttpAppError::NotFound),
    }
}

#[post("/api/person")]
async fn add_person(new_person: web::Json<Person>, data: web::Data<AppState>) -> AppResponse {
    let person = new_person.into_inner();
    let mut persons_guard = data.person_collection.write()?;
    let filtered = persons_guard.iter().any(|t| t.id == person.id);
    if !filtered {
        persons_guard.push(person);
        Ok(HttpResponse::Created().into())
    } else {
        Err(HttpAppError::Conflict)
    }
}

#[put("/api/person")]
async fn update_person(update_person: web::Json<Person>, data: web::Data<AppState>) -> AppResponse {
    let person = update_person.into_inner();
    let mut persons_guard = data.person_collection.write()?;
    let filtered = persons_guard.iter_mut().find(|t| t.id == person.id);
    match filtered {
        Some(p) => {
            p.age = person.age;
            p.date = person.date;
            p.name = person.name;
            Ok(HttpResponse::NoContent().into())
        }
        None => Err(HttpAppError::NotFound),
    }
}

#[delete("/api/person/{id}")]
async fn delete_person(path: web::Path<u32>, data: web::Data<AppState>) -> AppResponse {
    let id = path.into_inner();
    let mut persons_guard = data.person_collection.write()?;
    let index = persons_guard.iter().position(|t| t.id == id);
    match index {
        Some(index) => {
            persons_guard.remove(index);
            Ok(HttpResponse::NoContent().into())
        }
        None => Err(HttpAppError::NotFound),
    }
}
