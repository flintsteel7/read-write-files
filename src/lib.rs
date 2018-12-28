use std::io::{Error, ErrorKind};
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

pub struct File {
    pub filename: String,
    pub contents: String,
    pub path: PathBuf,
}

pub fn read_files(from: &str, with_ext: &str) -> Result<Vec<File>, Error> {
    let path = Path::new(from);
    let dir_entries = fs::read_dir(path)?;
    let mut file_data: Vec<File> = Vec::new();

    for entry_result in dir_entries {
        if let Ok(entry) = entry_result {
            let entry_path = entry.path();
            let extension = match entry_path.extension() {
                Some(ext) => ext.to_str().unwrap(),
                None => ""
            };
            if extension == with_ext || with_ext == "*" {
                if let Ok(file) = process_entry(entry) {
                    file_data.push(file);
                }
            }
        }
    }
    Ok(file_data)
}

pub fn write_files(file_data: Vec<File>) -> Vec<Result<String, (String, Error)>> {
    let mut results = Vec::new();
    for file in file_data {
        results.push(match fs::write(file.path, file.contents) {
            Ok(_) => Ok(file.filename),
            Err(error) => Err((file.filename, error))
        });
    }
    results
}

fn process_entry(entry: DirEntry) -> Result<File, Error> {
    let file_type = entry.file_type()?;
    if file_type.is_file() {
        // gather data in the formats we want
        let file_name = entry.file_name().into_string().unwrap();
        let file_contents = fs::read_to_string(entry.path())?;

        // build File struct
        return Ok(File {
            filename: file_name,
            contents: file_contents,
            path: entry.path(),
        });
    } else {
        return Err(Error::new(ErrorKind::Other, "not a file"))
    }
}