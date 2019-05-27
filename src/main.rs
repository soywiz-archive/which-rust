use std::fs::read_dir;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("which <executable>");
        exit(-1);
    }
    //println!("{:?}", args);
    let executable_to_search = &args[1];
    let paths_separator = if cfg!(windows) { ";" } else { ":" };
    let file_separator = if cfg!(windows) { "\\" } else { "/" };
    let path = env!("PATH");
    let paths = path.split(paths_separator);

    for path in paths {
        if let Ok(dir) = read_dir(path) {
            for try_file in dir {
                if let Ok(file) = try_file {
                    if let Some(file_name) = file.file_name().to_str() {
                        let file_name_lc = file_name.to_lowercase();
                        if file_name_lc.ends_with(".exe") || file_name_lc.ends_with(".cmd") || file_name_lc.ends_with(".bat") {
                            let base = file_name_lc.substring_before_last(".");
                            if base == executable_to_search {
                                //println!("Name {} : {}", file_name, base);
                                println!("{}{}{}", path, file_separator, file_name);
                                exit(0);
                            }
                        }
                    }
                }
            }
        } else {
            println!("Can't read {}", path);
        }
    }
    exit(1);
}

trait StringExt {
    fn substring_before_last(&self, separator: &str) -> &str;
}

impl StringExt for str {
    fn substring_before_last(&self, separator: &str) -> &str {
        if let Some(z) = self.rfind(separator) {
            return &self[0..z];
        } else {
            return self;
        }
    }
}
