use crate::libs::parse_config::Profile;
use crate::libs::load_config::load_profile;
use actix_web::{get, web};

#[get("/profile")]
async fn get_profile() -> web::Json<Profile> {
    match load_profile() {
        Ok(profile) => web::Json(profile),
        Err(_) => panic!("Failed to load profile."),
    }
}