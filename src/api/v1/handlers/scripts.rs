use actix_web::{web, HttpResponse};
use crate::script_manager;

pub async fn list_scripts(path: web::Path<String>) -> HttpResponse {
    let script_manager = script_manager::get_script_manager();
    let path_str = path.into_inner();
    let path = if path_str.is_empty() { None } else { Some(path_str.as_str()) };
    
    match script_manager.list_scripts(path) {
        Ok(script_list) => HttpResponse::Ok().json(script_list),
        Err(_) => HttpResponse::InternalServerError().body("Failed to list scripts"),
    }
}

pub async fn list_scripts_recursive() -> HttpResponse {
  println!("get list of scripts 1");
  let script_manager = script_manager::get_script_manager();

  println!("get list of scripts 2");

  match script_manager.list_scripts_recursive() {
    Ok(script_list) => HttpResponse::Ok().json(script_list),
    Err(err) => {
      let error_message = format!("Failed to list scripts: {}", err);
      HttpResponse::InternalServerError().body(error_message)
    } 
  }
}
