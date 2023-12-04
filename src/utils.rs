use std::fs;

pub fn read_file(path_to_file: &'static str) -> String {
    let input_contents = fs::read_to_string(path_to_file)
        .expect("Should have been able to read the file. Try running the code in the root directory of the development project.");

    input_contents
}
