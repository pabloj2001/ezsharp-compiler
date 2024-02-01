use std::{env, fs::{self, OpenOptions}, io::Write};

pub trait Loggable {
    fn to_log_message(&self) -> String;
}

pub struct FileLogAttributes {
    filename: String,
    append: bool,
}

impl FileLogAttributes {
    pub fn new(filename: String, append: bool) -> Self {
        FileLogAttributes { filename, append }
    }
}

pub fn log_to_file<T>(loggable: &T, attributes: &FileLogAttributes) -> Result<(), String>
where
    T: Loggable,
{
    // Create the log folder if it doesn't exist
    let log_folder = &attributes.filename.split("/").collect::<Vec<&str>>();
    let log_folder = log_folder[..log_folder.len() - 1].join("/");
    let path = env::current_dir().map_err(|e| e.to_string())?;
    let path = path.join(log_folder);
    fs::create_dir_all(path).map_err(|e| e.to_string())?;

    // Open file
    let mut file = OpenOptions::new()
        .append(attributes.append)
        .create(true)
        .write(true)
        .open(&attributes.filename)
        .map_err(|e| e.to_string())?;

    // Clear the file if it's not being appended to
    if !attributes.append {
        file.set_len(0).map_err(|e| e.to_string())?;
    }

    // Write to file
    file.write_all(loggable.to_log_message().as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}