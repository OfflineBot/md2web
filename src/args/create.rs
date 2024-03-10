use std::io::{Read, Write};

use md2html::{Settings, md_to_html};
use crate::Files;
pub fn create(main_path: String, target: String) {
    let mut files = Files::new(main_path.clone());
    files.get_files();
    files.filter();



    let settings = Settings::default();

    for file in files.files.iter() {

        let mut current_body = String::new();
        let mut f = std::fs::File::open(file).unwrap();
        f.read_to_string(&mut current_body).unwrap();

        let full_location = format!(
            "{}/{}/{}", 
            &main_path, 
            target, 
            file.replace(&main_path, "").replace("//", "/").replace(".md", ".html")
        ).replace("//", "/");
        let relative_location = format!(
            "{}/{}",
            target,
            file.replace(&main_path, "").replace("//", "/").replace(".md", "html")
        ).replace("//", "/");

        let title_split = file.split("/").collect::<Vec<&str>>();
        let title = title_split[title_split.len() - 1].replace(".md", "").replace(".html", "");
        let mut file = std::fs::File::create(full_location).unwrap();
        file.write_all(
            md_to_html(
                current_body, 
                title, 
                relative_location, 
                main_path.clone(), 
                Some(settings.clone()), 
                files.files.clone()
            ).as_bytes()
        ).unwrap();
    }
}