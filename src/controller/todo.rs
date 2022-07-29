use super::controller::Controller;
pub struct TodoController {
    base_url: String,
}

impl TodoController {
    pub fn new() -> TodoController {
        return TodoController {
            base_url: String::from("/api/todo/"),
        };
    }
}

impl Controller for TodoController {
    fn base(&self) -> &str {
        return self.base_url.as_str();
    }
}
#[get("/load")]
pub fn load_todo() -> &'static str {
    return "Hello";
}
