use std::fs;
use dirs;

// Here define a method to move log file under directory to a specific directory
fn mover(log_dir: &str, target_dir: &str) {
    // Check if target_dir exists, if not, create it
    println!("Checking log directory {}", fs::metadata(log_dir).is_ok());

     // Check if target_dir exists, if not, create it
    if let Err(_) = fs::metadata(target_dir) {
        fs::create_dir(target_dir).expect("Unable to create target directory");
        println!("Created target directory {}", target_dir);
    }

    // Get all files end with .log under log_dir
    let files = std::fs::read_dir(log_dir).unwrap();
    let mut count = 0;
    for file in files {
        let file_name = file.unwrap().path().display().to_string();
        // Remove the path and get the file name
        let file_name_str = file_name.split("/").last().unwrap();
        println!("File name: {}", file_name_str);
        if file_name_str.ends_with(".log") {
            count += 1;
            // Move file to target_dir
            let target_file_name = format!("{}/{}", target_dir, file_name_str);
            println!("Moving {} to {}", file_name_str, target_file_name);
            fs::rename(file_name, target_file_name).expect("Unable to move file");
        }
    }
    println!("Moved {} files", count);
}

fn main() {
      // Get the home directory
    if let Some(home_dir) = dirs::home_dir() {
        let log_dir = home_dir.join("java-project");
        let target_dir = home_dir.join("java-project/log");

        println!("Start to move log files from {} to {}", log_dir.display(), target_dir.display());
        mover(log_dir.to_str().unwrap(), target_dir.to_str().unwrap());
    } else {
        eprintln!("Unable to determine the home directory.");
    }
}
