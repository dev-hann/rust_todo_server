use rocket::Route;
use crate::handlers::todo::{index,todos,create_todo,delete_todo};

 pub   fn routes() -> Vec<Route> {
        routes![index,todos,create_todo,delete_todo]
    }