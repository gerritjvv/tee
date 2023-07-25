use std::fs::File;
use std::{fs, io};
use std::io::{Read, Write};
use chrono::Utc;
use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // File size in bytes at which the file is renamed and a new file is written
    #[arg(short='s', long, default_value_t = 134217728)]
    file_size_limit: u64,

    // The number of files to keep
    #[arg(short ='l', long, default_value_t = 2)]
    file_count_limit: u32,

    // If false the data is not written to stdout
    #[arg(short, long, default_value_t = true)]
    write_to_stdout: bool,

    // The file to write to
    file: String,
}

struct Context {
    file_size_limit: u64,
    file_count_limit: u32,

    current_file_path: String,
    current_file: File,
}

fn main() -> () {
    let args = Args::parse();

    const BUFFER_SIZE: usize = 1024;
    let mut buffer = [0u8; BUFFER_SIZE];

    let stdin = io::stdin();

    let mut stdin_lock = stdin.lock();
    let context = create_context(&args).unwrap();

    /*
      read a buffer at a time copying the output to the current log file and stdout
      Note: we could use a text buffer but there is no need to handle text while we are writing to the log file
     */
    let mut loop_context = context;
    loop {

        match stdin_lock.read(&mut buffer) {
            Ok(bytes_read) => {
                match process_input(&mut loop_context, &mut buffer, bytes_read) {
                    None => {}
                    Some(ctx) => {
                        loop_context = ctx;
                    }
                }
            }
            Err(err) => {
                println!("Error reading from stdin {:?}", err);
                break;
            }
        }
    }
}

// encapsulates the process of calling the bytes handler to write to file and check for rotation
fn process_input(context: &mut Context, buffer: &mut [u8; 1024], bytes_read: usize) -> Option<Context> {
    if bytes_read > 0 {
        let context2 = context;
        handle_std_bytes(context2, &buffer, bytes_read).expect("Cannot write to file");
        match check_and_rotate(context2).unwrap() {
            None => {}
            Some(context_new) => {
                return Some(context_new);
            }
        }
    }
    return None;
}

// check if we need to rotate (roll) a file, if so, the file is renamed and a new file is created and set to the application context
fn check_and_rotate(context: &mut Context) -> Result<Option<Context>, io::Error> {
    let file_metadata = context.current_file.metadata().unwrap();
    let file_size = file_metadata.len();

    if file_size > context.file_size_limit {
        // rotate
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

// when rolling a file, this file name is used
fn roll_file_name_from(path: &String) -> String {
    let (name, extension) = match path.rfind('.') {
        Some(index) => path.split_at(index),
        None => (path.as_str(), "")
    };

    let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let roll_file_name = format!("{}_{}{}", name, timestamp, extension);

    return roll_file_name;
}

// create an application context from the arguments provided
fn create_context(args: &Args) -> io::Result<Context> {
    let file_path = args.file.clone();

    let current_file_res = File::create(file_path);

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
        current_file_path: args.file.clone(),
        current_file,
        file_size_limit: args.file_size_limit,
        file_count_limit: args.file_count_limit,
    });
}

// write to console and the current output file
fn handle_std_bytes(context: &mut Context, buff: &[u8; 1024], len: usize) -> io::Result<()> {
    println!("{}", String::from_utf8_lossy(buff));
    return context.current_file.write_all(&buff[..len]);
}
