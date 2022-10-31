use manager::ManagerError;
use manager::{init_loggers, run};

fn main() -> Result<(), ManagerError> {
    init_loggers();
    run()
}
