use suborbital::runnable::*;
use suborbital::req;
use suborbital::db;
use suborbital::db::query;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    title: String,
    is_complete: bool 
}

struct UpdateTodo{}

impl Runnable for UpdateTodo {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");

        let key = req::url_param("id");
        let in_string = String::from_utf8(input).unwrap();

        let todo: Todo = serde_json::from_str(&in_string).unwrap();

        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("title", todo.title.as_str()));
        query_args.push(query::QueryArg::new("is_complete", (format!("{}", todo.is_complete)).as_str()));
        query_args.push(query::QueryArg::new("id", key.as_str()));

        match db::update("UpdateTodo", query_args) {
            Ok(result) => Ok(result),
            Err(e) => {
                Err(RunErr::new(500, e.message.as_str()))
            }
        }
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &UpdateTodo = &UpdateTodo{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
