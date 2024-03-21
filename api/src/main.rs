use log::info;
use pretty_env_logger;

fn main() {
    pretty_env_logger::init();
    info!("This is an information log");
}
