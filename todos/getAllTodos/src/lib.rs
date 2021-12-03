use suborbital::runnable::*;
use suborbital::db;

struct GetAllTodos{}

impl Runnable for GetAllTodos {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");

        match db::select("SelectAllTodos", Vec::new()) {
            Ok(result) => Ok(result),
            Err(e) => {
                Err(RunErr::new(500, e.message.as_str()))
            }
        }
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &GetAllTodos = &GetAllTodos{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
