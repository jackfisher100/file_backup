use std::fs;
use std::path::Path;
use std::io;



fn main() {
    let mut counter = 0;
    get_files(Path::new("C:"), &mut counter);
    println!("{}", counter);
}


fn get_files(dir: &Path, file_count: &mut u64) {

    match fs::read_dir(dir) {
        Ok(files) => {
            for file in files{
                let entry = file.unwrap();
                let path = entry.path();
                if path.is_dir(){
                    get_files(&path, file_count)
                }
                else{
                    *file_count += 1;
                    println!("{:?}: {}", file_count, path.to_str().unwrap());
                }
            }
        },
        Err(_) => {println!("Failed ###")},
    };

}
