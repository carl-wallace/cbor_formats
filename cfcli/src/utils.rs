use std::ffi::OsStr;
use std::path::Path;
use walkdir::WalkDir;

pub(crate) fn find_files(dir: &String, ext: &str, list: &mut Vec<String>) {
    if !Path::is_dir(Path::new(dir)) {
        return;
    }

    for entry in WalkDir::new(dir) {
        match entry {
            Ok(e) => {
                let path = e.path();
                if e.file_type().is_dir() {
                    if let Some(s) = path.to_str() {
                        if s != dir {
                            let count = list.len();
                            find_files(&s.to_string(), ext, list);
                            if count == list.len() {
                                continue;
                            }
                        }
                    }
                    continue;
                } else {
                    let file_exts = vec![ext];
                    if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                        if !file_exts.contains(&ext) {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    if let Some(s) = path.to_str() {
                        let s = s.to_string();
                        if !list.contains(&s) {
                            list.push(s);
                        }
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }
}
