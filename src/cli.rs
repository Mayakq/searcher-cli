use clap::{arg, command};

#[derive(Debug)]
pub enum TypeSearch {
    Absolute,
    Relative,
}

/// settings for search
#[derive(Debug)]
pub struct Settings {
    pub file: String,
    pub path: String,
    pub system_dir: bool,
    pub informed: bool,
    pub type_search: TypeSearch,
    pub ignore_case: bool,
}

impl Settings {
    fn init(
        file: String,
        path: String,
        system_dir: bool,
        informed: bool,
        type_search: TypeSearch,
        ignore_case: bool,
    ) -> Settings {
        Settings {
            file,
            path,
            system_dir,
            informed,
            type_search,
            ignore_case,
        }
    }
}

pub fn init_cli() -> Settings {
    let matched = command!()
        .arg(arg!(-i --informed  "Receive the debug of information. Default no"))
        .arg(arg!(-p --path <PATH> "Where to start the search. The default is the current directory and subfolders").default_value("./"))
        .arg(arg!(-f --file <FILE> "File for search").required(true))
        .arg(arg!(-t --type_search "Type search. Default absolute if flag set then relative."))
        .arg(arg!(-s --system_dir "Search in system directory. Default false"))
        .arg(arg!(-u --case "Ignore case. Example if false then file.txt != FILE.txt"))
        .get_matches();
    let mut type_search: TypeSearch = TypeSearch::Absolute;
    let flag_search = matched.get_flag("type_search");
    let ignore_case = matched.get_flag("case");
    let informed = matched.get_flag("informed");
    let path = matched.get_one::<String>("path").unwrap().clone();
    let system_dir = matched.get_flag("system_dir");
    let file: String;

    if let Some(file_to_search) = matched.get_one::<String>("file") {
        file = file_to_search.clone();
    } else {
        panic!("Name file to search not set")
    }
    if flag_search {
        type_search = TypeSearch::Relative
    }
    Settings::init(file, path, system_dir, informed, type_search, ignore_case)
}
