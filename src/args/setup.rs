use crate::Files;
use std::io::Write;
use super::css_template;

fn create_css(main_path: &String, target: &String) {
    let css_body = css_template();

    let mut css = format!("{}/{}/css", main_path, target).replace("///", "/").replace("//", "/");
    std::fs::create_dir_all(&css).unwrap();
    css += "/styles.css";
    let mut css_output = std::fs::File::create(css).unwrap();
    css_output.write_all(css_body.as_bytes()).unwrap();
}

pub fn setup(main_path: String, target: String) {
    let mut files = Files::new(main_path.clone()); 
    files.get_files();
    files.create_folder_tree(&target);
    create_css(&main_path, &target);
}