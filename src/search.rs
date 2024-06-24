use walkdir::WalkDir;

use crate::cli::{init_cli, Settings, TypeSearch};

struct Search {}

pub fn start_search() {
    let settings = &init_cli();
    Search::search(settings);
}

impl Search {
    fn get_walkdir(settings: &Settings) -> WalkDir {
        WalkDir::new(&settings.path)
            .same_file_system(settings.system_dir)
    }
    fn search(settings: &Settings) {
        let walkdir = Search::get_walkdir(settings);
        for dir in walkdir {
            match dir {
                Ok(dir_entry) => match settings.type_search {
                    TypeSearch::Absolute => {
                        let mut file_name = dir_entry.file_name().to_str().unwrap().to_string();
                        check_case_symbol(settings, &mut file_name);
                        if file_name == settings.file.to_lowercase() {
                            println!("{:?} {:?}", file_name, dir_entry.path());
                        }
                    }
                    TypeSearch::Relative => {
                        let mut file_name = dir_entry.file_name().to_str().unwrap().to_string();
                        check_case_symbol(settings, &mut file_name);
                        if file_name.contains(&settings.file) {
                            println!("{:?} {:?}", file_name, dir_entry.path());
                        }
                    }
                },
                Err(error) => {
                    if settings.informed {
                        eprintln!("{}", error)
                    }
                }
            }
        }
        fn check_case_symbol(settings: &Settings, file_name: &mut String) {
            if settings.ignore_case {
                for char in file_name.chars() {
                    if !char.is_lowercase() {
                        *file_name = file_name.to_lowercase();
                        break;
                    }
                }
            }
        }
    }
}
