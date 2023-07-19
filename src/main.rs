use std::fs::File;
use std::{fs, io};
use std::io::{Read, Write};
use chrono::Utc;

struct Context {
    file_size_limit: u64,
    file_count_limit: u32,

    current_file_path: String,
    current_file: File,
}

fn main() -> () {
    const BUFFER_SIZE: usize = 1024;
    let mut buffer = [0u8; BUFFER_SIZE];

    let stdin = io::stdin();

    let mut handle = stdin.lock();

    let mut context = create_context(String::from("/tmp/file.log")).unwrap();


    loop {
        match handle.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let context2 = &mut context;
                    handle_std_bytes(context2, &buffer, bytes_read).expect("Cannot write to file");
                    match check_and_rotate(context2).unwrap() {
                        None => {}
                        Some(context_new) => {
                            context = context_new;
                        }
                    }
                }
            }
            Err(err) => {
                println!("Error reading from stdin {:?}", err);
                break;
            }
        }
    }


    // read std
    // write file
    // write stdout

    // rotate check

    println!("Hello, world!");
}

fn check_and_rotate(context: &mut Context) -> Result<Option<Context>, std::io::Error> {
    let file_metadata = context.current_file.metadata().unwrap();
    let file_size = file_metadata.len();

    if file_size > context.file_size_limit {
        // rotate
        println!("Rotatefile: file_size {} ", file_size);

        let roll_file_name = roll_file_name_from(&context.current_file_path);

        fs::rename(&(context.current_file_path), &roll_file_name)?;

        let file = File::create(&(context.current_file_path))?;
        let context2 = Context {
            file_size_limit: context.file_size_limit,
            file_count_limit: context.file_count_limit,
            current_file_path: String::from(context.current_file_path.as_str()),
            current_file: file,
        };
        return Ok(Some(context2));
    }


    return Ok(None);
}

fn roll_file_name_from(path: &String) -> String {
    let (name, extension) = match path.rfind('.') {
        Some(index) => path.split_at(index),
        None => (path.as_str(), "")
    };

    let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let roll_file_name = format!("{}_{}{}", name, timestamp, extension);

    return roll_file_name;
}

fn create_context(file_path: String) -> io::Result<Context> {
    let current_file_res = File::create(file_path.clone());

    match current_file_res {
        Ok(_) => {
            // ignore
        }
        Err(err) => {
            return Err(err);
        }
    }

    let current_file = current_file_res.unwrap();

    return Ok(Context {
        current_file_path: file_path.clone(),
        current_file,
        file_size_limit: 1024 * 1024 * 128,
        file_count_limit: 3,
    });
}

fn handle_std_bytes(context: &mut Context, buff: &[u8; 1024], len: usize) -> io::Result<()> {
    return context.current_file.write_all(&buff[..len]);
}
