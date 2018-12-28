# read-write-files

## Tiny Rust library for reading and writing files with the indicated extension from/to the specified directory

I do a lot of reading all the files of a certain type in a directory, transforming their contents, then writing those files back, and this little library helps me do it.

### Functions

```Rust
read_files(from: &str, with_ext: &str)
```

```Rust
write_files(files: Vec<File>)
```
`read_files()` Does what you'd expect: reads the files whose extensions match `with_ext` from the `from` path

Returns a vector containing `File` structs:
```Rust
pub struct File {
    pub filename: String,
    pub contents: String,
    pub path: PathBuf,
}
```
You can pass `"*"` as the `with_ext` argument to `read_files()` to match all file extensions, however, the library will only read and modify files with UTF-8 encoding.


`write_files()` takes a vector of `File` structs and writes their `contents` to their `path`

### Example

The following example will read all files with the extension `txt` from the parent directory and overwrite their contents with "Some data"
```Rust
fn main() {
    let path = "../";
    let error = format!("Could not read {:?}", path);
    let file_data = read_files(path, "txt").expect(&error);
    let mut changed_file_data: Vec<File> = Vec::new();
    for file in file_data {
        changed_file_data.push(change_file_contents(file));
    }
    for result in write_files(changed_file_data) {
        match result {
            Ok(filename) => println!("Changes written to {:?}", filename),
            Err((filename, error)) => println!("Encountered a problem writing {:?}: {:?}", filename, error)
        };
    }
}

fn change_file_contents(file: File) -> File {
    let new_contents = String::from("Some data");
    File { contents: new_contents, ..file}
}
```
