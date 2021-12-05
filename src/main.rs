use notify_rust::Notification;
use std::fs;
use std::path::Path;

fn main() {
    #[cfg(not(debug_assertions))]
        Notification::new()
        .summary("Delete Node Modules")
        .body("Started deletion of node modules folder")
        .show()
        .unwrap();
    let mut counter = 0;
    check_folder(
        Path::new("./"),
        &mut counter,
    );
    #[cfg(not(debug_assertions))]
    Notification::new()
        .summary("Delete Node Modules")
        .body(
            format!(
                "Successfully deleted {} node_modules folders",
                counter
            )
            .as_str(),
        )
        .show()
        .unwrap();
}

fn check_folder(path: &Path, counter: &mut i32) {
    if path.to_path_buf().is_dir() {
        if path
            .canonicalize()
            .unwrap()
            .file_name()
            .unwrap()
            .eq("node_modules")
        {
            println!(
                "deleted node modules in {:?}",
                fs::canonicalize(&path.to_path_buf()).unwrap()
            );
            fs::remove_dir_all(path).unwrap();
            *counter += 1;
        } else {
            for p in path.read_dir().unwrap() {
                match p {
                    Ok(t) => {
                        check_folder(t.path().as_path(), counter);
                    }
                    Err(e) => {
                        println!("ERR: {}", e);
                    }
                }
            }
        }
    }
}