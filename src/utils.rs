use std::fs;
pub fn parsefile(filename: &str) -> Vec<String> {
    let lines = fs::read_to_string(filename).expect("Unable to read file");
    let linevec = lines
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    linevec
}
