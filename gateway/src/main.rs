use log::{debug, error, info};
use libs::{logging};

fn main() {
    logging::init();

    info!("This is an info log");
    error!("This is an error log");
    debug!("This is an debug log");
}
