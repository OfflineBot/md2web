
use super::{create, info, setup};

fn clean(main_path: String, target: String) {
    std::fs::remove_dir_all(format!("{}/{}", main_path, target).replace("//", "/")).unwrap();
}

pub fn handle_args() -> std::io::Result<()> {
    let argc = std::env::args().len();
    let argv = std::env::args().collect::<Vec<String>>();

    let main_path = std::env::current_dir().unwrap().to_str().unwrap().to_string().replace("\\\\", "/").replace("\\", "/");
    let target = String::from("output");

    match argc {
        0 => println!("Something went wrong!"),
        1 => info(),
        2 => {
            if argv[1].trim() == "setup" {
                setup(main_path, target);
            } else if argv[1].trim() == "create" {
                create(main_path, target)
            } else if argv[1].trim() == "clean" {
                clean(main_path, target);
            } else {
                println!("invalid argument");
            }
        }, 
        _ => println!("Invalid amount of arguments"),
    }

    Ok(())
}