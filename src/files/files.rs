
pub struct Files {
    pub main_path: String,
    pub files: Vec<String>,
    pub folders: Vec<String>,
}

impl Files {
    pub fn new(main_path: String) -> Self {
        let main = main_path.replace("\\\\", "/").replace("\\", "/");
        Files { main_path: main, files: Vec::new(), folders: Vec::new() }
    }

    pub fn get_files(&mut self) {
        let mut folders = Vec::new();
        folders.push(self.main_path.clone());
        let mut files = Vec::new();
        let mut running = true;

        while running {
            let mut new_folders = folders.clone();
            let mut new_files = files.clone();

            for folder in folders.iter() {
                let data = std::fs::read_dir(folder).unwrap();
                for entry in data {
                    let entry = entry.unwrap();
                    let entry_name = format!("{}/{}", folder, entry.file_name().to_str().unwrap().to_string()).replace("//", "/");
                    if entry.metadata().unwrap().is_file() {
                        if !new_files.contains(&entry_name) {
                            new_files.push(entry_name);
                        }
                    } else if entry.metadata().unwrap().is_dir() {
                        if !new_folders.contains(&entry_name) {
                            new_folders.push(entry_name);
                        }
                    }
                }
            }

            if new_files == files {
                println!("Found {} Files", files.len());
                running = false;
            }

            files = new_files;
            folders = new_folders;
        }

        self.files = files;
        self.folders = folders;
    }
    
    pub fn create_folder_tree(self, target: &String) {
        for folder in self.folders.iter() {
            let folder_name = folder.replace(&self.main_path.clone(), "");
            std::fs::create_dir_all(
                format!("{}/{}/{}", self.main_path.clone(), target, folder_name)
            ).unwrap();
        }
    }
    // get all .md files
    pub fn filter(&mut self) {
        let mut file_vec: Vec<String> = Vec::new(); 
        for file in self.files.iter() {
            if file.ends_with(".md") {
                file_vec.push(file.to_owned());
            }
        }
        self.files = file_vec;
    }

    pub fn set_index(&mut self, index: String) {
        let mut out: Vec<String> = Vec::new();
        let search = index + ".md";
        for file in self.files.iter() {
            if file.contains(search.as_str()) {
                out.push(file.replace(&search, "index.md"));
            }
        }  
        self.files = out;
    } 

    pub fn print_files(&self) {
        println!("{:#?}", self.files);
    }

}