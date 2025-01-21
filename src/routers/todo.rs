use rocket::Route;
use crate::handlers::todo::{index,todos,create_todo,delete_todo,update_todo,get_todo};

 pub fn routes() -> Vec<Route> {
    routes![index,todos,create_todo,delete_todo,update_todo,get_todo]
}
