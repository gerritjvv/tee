use std::fs::File;
use std::io;
use std::io::{Read, Write};


struct Context {
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
                    handle_std_bytes(&mut context, &buffer, bytes_read).expect("Cannot write to file");
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

fn create_context(file_path: String) -> io::Result<Context> {
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
        current_file,
    });
}

fn handle_std_bytes(context: &mut Context, buff: &[u8; 1024], len: usize) -> io::Result<()> {
    return context.current_file.write_all(&buff[..len]);
}
