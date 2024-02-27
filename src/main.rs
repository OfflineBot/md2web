mod args;
mod files;
use args::handle_args;
pub use files::Files;

fn main() {
    handle_args().unwrap(); 
}
