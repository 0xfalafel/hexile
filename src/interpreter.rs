use std::{fs::{read_to_string, File}, io::BufReader, path::PathBuf};

mod tokenizer;
use tokenizer::Tokenizer;


fn open_file(filepath: &PathBuf) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

pub fn binary_template(template_path: PathBuf, file_path: PathBuf) {
    let template = match read_to_string(&template_path) {
        Err(_) => {
            eprintln!("Failed to open {}", &template_path.display());
            return;
        },
        Ok(template) => template,
    };

    let file = open_file(&file_path)
        .map_err(|_| {
            eprintln!("Failed to open {}", &file_path.display());
            return;
        });

    println!("{}", template);
    let tokenizer = Tokenizer::new(template);
}