use rand::Rng;
use std::cmp::Ordering;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const ROOT_FOLDER: &str = "game";

fn create_files(secret_number: &i32) {
    for i in 1..=100 {
        let formatted_dir_path = format!("{}/{}", ROOT_FOLDER, i);

        let dir_path = Path::new(&formatted_dir_path);

        fs::remove_dir_all(&dir_path).ok();
        fs::create_dir_all(&dir_path).expect("failed to create dir");

        let file_name = match i.cmp(&secret_number) {
            Ordering::Less => "Go bigger",
            Ordering::Equal => "YOU FOUND IT",
            Ordering::Greater => "Go lower",
        };

        let formatted_file_path = format!("{}/{}", formatted_dir_path, file_name);
        let file_path = Path::new(&formatted_file_path);

        fs::File::create(&file_path).expect("failed to create file");
    }
}

fn create_solution_enter_file(secret_number: &i32) {
    let binary_code = format!(
        r#"
        use std::fs;
        use std::io;

        fn main() {{
            let secret_number: i32 = "{}".parse().expect("failed to parse secret number");
            
            loop {{
                let mut guess = String::new();

                println!("Enter secter number: ");

                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");

                let guess: i32 = match guess.trim().parse() {{
                    Ok(num) => num,
                    Err(_) => continue,
                }};

                if guess == secret_number {{
                    println!("It's correct, grats!");
                    break;
                }} else {{
                    println!("It's wrong, try again");
                }}
            }}

            fs::remove_dir_all("{}").ok();
            fs::remove_file("solution").ok();
        }}"#,
        secret_number, ROOT_FOLDER
    );

    let file_path = Path::new("solution.rs");

    let mut file = fs::File::create(&file_path).expect("failed to create solution.rs");
    file.write_all(binary_code.as_bytes())
        .expect("failed to write binary code");

    Command::new("rustc")
        .arg("solution.rs")
        .output()
        .expect("failed to compile solution.rs");

    fs::remove_file("solution.rs").expect("failed to delete solution.rs");
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    create_files(&secret_number);
    create_solution_enter_file(&secret_number);
}
