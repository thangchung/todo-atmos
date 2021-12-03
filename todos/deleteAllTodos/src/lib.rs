use suborbital::runnable::*;
use suborbital::db;

struct DeleteAllTodos{}

impl Runnable for DeleteAllTodos {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");

        match db::delete("DeleteAllTodos", Vec::new()) {
            Ok(result) => Ok(result),
            Err(e) => {
                Err(RunErr::new(500, e.message.as_str()))
            }
        }        
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &DeleteAllTodos = &DeleteAllTodos{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
