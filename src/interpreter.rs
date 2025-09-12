use std::{fs::File, io::BufReader, path::PathBuf};
// pub mod interpreter;

fn open_file(filepath: &PathBuf) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

pub fn binary_template(template_path: PathBuf, file_path: PathBuf) {
    println!("Hi mom!");

    let template = open_file(&template_path)
        .map_err(|_| {
            eprintln!("Failed to open {}", &template_path.display());
            return;
        });

    let file = open_file(&file_path)
        .map_err(|_| {
            eprintln!("Failed to open {}", &file_path.display());
            return;
        });

}