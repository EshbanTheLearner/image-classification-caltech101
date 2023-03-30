use std::io;
use std::fs::{ self, DirEntry, copy, create_dir_all };
use std::path::Path;

const DATASET_FOLDER: &str = "dataset";

fn visit_dir(dir: &Path, train_fn: &dyn Fn(&DirEntry), test_fn: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        let mut this_label = String::from("");
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dir(&path, train_fn, test_fn)?;
            } else {
                let full_path: Vec<String> = path.to_str().unwrap()
                    .split("/").into_iter()
                    .map(|x| x[..].to_string()).collect();
                if this_label == full_path[1] {
                    train_fn(&entry);
                } else {
                    test_fn(&entry);
                }
                this_label = full_path[1].clone();
            }
        }
    }
    Ok(())
}

fn print_directory(dir: &Path) {
    println!("{:?}", dir);
}

fn move_file(from_path: &DirEntry, to_path: &Path) -> io::Result<()> {
    let root_folder = Path::new(DATASET_FOLDER);
    let second_order = root_folder.join(to_path);
    let full_path: Vec<String> = from_path.path().to_str().unwrap()
        .split("/").into_iter()
        .map(|x| x[..].to_string()).collect();
    let label = full_path[1].clone();
    let third_order = second_order.join(label);
    create_dir_all((&third_order))?;
    let filename = from_path.file_name();
    let to_filename = third_order.join(&filename);
    copy(from_path.path(), to_filename)?;
    Ok(())
}