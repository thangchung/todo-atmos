use suborbital::runnable::*;
use suborbital::db;
use suborbital::db::query;
use suborbital::log;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    title: String,
    is_complete: bool 
}

struct CreateTodo{}

impl Runnable for CreateTodo {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        
        let in_string = String::from_utf8(input).unwrap();

        let todo: Todo = serde_json::from_str(&in_string).unwrap();

        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("title", todo.title.as_str()));
        query_args.push(query::QueryArg::new("is_complete", (format!("{}", todo.is_complete)).as_str()));

        let msg = format!("is_complete: {}", todo.is_complete);
        log::info(msg.as_str());

        match db::insert("CreateTodo", query_args) {
            Ok(result) => Ok(result),
            Err(e) => {
                Err(RunErr::new(500, e.message.as_str()))
            }
        }    
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &CreateTodo = &CreateTodo{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
