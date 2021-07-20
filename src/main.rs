
use std::fs;
use std::path::Path;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use ftp::FtpStream;
#[allow(unused_imports)]
use std::time::SystemTime;


const LOCAL_BASE: &str = "D:\\Documents\\Computing\\Rust\\file_backup";
const FNAS_BASE: &str = "//fnas2/FTP/Jack/Documents/Computing/Rust/file_backup";
fn main() {

    let mut total_counter = 0;
    let mut live_counter = 0;
    count_files("\\", &mut total_counter).unwrap();
    update_files("\\", &mut live_counter, total_counter).unwrap();

}


fn count_files(dir: &str, file_count: &mut u64) -> std::io::Result<()> {
    
    match fs::read_dir(Path::new(&format!("{}{}", LOCAL_BASE, dir))) {
        Ok(local_files) => {

            for file in local_files{
                let local_path = file.unwrap().path();
                if local_path.is_dir(){
                    count_files(&remove_starting_directory(local_path.to_str().unwrap()), file_count)?
                }
                else{
                    *file_count += 1;
                }
                
            }
        }
            Err(err) => {println!("Permission denied: {}", err)},  
    };
    Ok(())
}

fn update_files(dir: &str, file_count: &mut u64, total_count: u64) -> std::io::Result<()> {

    match fs::read_dir(Path::new(&format!("{}{}", LOCAL_BASE, dir))) {
        Ok(local_files) => {

            for file in local_files{
                let file = file.unwrap();
                let local_path = file.path();

                let fnas_dir = &format!("{}{}\\{}", FNAS_BASE, dir, file.file_name().to_str().unwrap());
                let fnas_path = Path::new(fnas_dir);
                
                if local_path.is_dir(){

                    if fnas_path.exists() == false {
                        match fs::create_dir(fnas_path) {
                            Ok(_) => println!("{}% - Created new folder: {}", *file_count*100/total_count, fnas_path.to_str().unwrap()), 
                            Err(error) => println!("Folder: {}: {}", error, local_path.to_str().unwrap()),

                        }
                    }


                    update_files(&remove_starting_directory(local_path.to_str().unwrap()), file_count, total_count)?
                }
                else{
                    if fnas_path.exists() == false {
                        match fs::copy(&local_path, &fnas_path) {
                            Ok(_) => println!("{}% - Created file: {}", *file_count*100/total_count, fnas_path.to_str().unwrap()), 
                            Err(error) => println!("File: {}: {}", error, local_path.to_str().unwrap()),
                        }
                    }
                    *file_count += 1; 
                }
                
            }
        }
            Err(err) => {println!("Permission denied: {}", err)},  
    };
    Ok(())
}


fn remove_starting_directory(ending: &str) -> &str {
    let result = &ending[LOCAL_BASE.len()..];

    result

}
