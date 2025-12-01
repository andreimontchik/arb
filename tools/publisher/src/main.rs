use {
    common::serializer::{BinarySerializer, CsvSerializer, Serializer, BUFFER_SIZE},
    memmap2::MmapOptions,
    std::{
        env,
        fs::OpenOptions,
        io::{BufRead, BufReader},
        thread,
        time::Duration,
    },
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Invalid startup arguments! Usage: publisher <MESSAGE_FILE> <MMAP_FILE>");
    }

    let src_file_name = &args[1];
    println!("The message file: ({src_file_name})");

    let dest_file_name = &args[2];
    println!("The mmap file: ({dest_file_name})");

    let src_file_handle = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(&src_file_name)
        .expect(&format!("Failed to open the message file ({})!", src_file_name));

    let dest_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(dest_file_name)
        .unwrap();
    dest_file.set_len(BUFFER_SIZE as u64).unwrap();
    let mut dest_mmap = unsafe { MmapOptions::new().len(BUFFER_SIZE).map_mut(&dest_file) }.unwrap();

    println!("Processing the message file...");
    let file_reader = BufReader::new(src_file_handle);
    let mut csv_serializer = CsvSerializer::new();
    let mut binary_serializer = BinarySerializer::new();
    let mut msg_buffer: Vec<u8> = vec![0; BUFFER_SIZE];
    for (index, line) in file_reader.lines().enumerate() {
        match line {
            Ok(line) => match csv_serializer.deserialize_message(line.as_bytes()) {
                Ok(msg) => match binary_serializer.serialize_message(&msg, &mut msg_buffer) {
                    Ok(_) => {
                        println!("Publishing the message {} ({}).", index, msg);
                        dest_mmap.copy_from_slice(&msg_buffer[..]);
                    }
                    Err(err) => eprintln!("Failed to serialize the msg {}! {:?}", msg, err),
                },
                Err(err) => eprintln!("Failed to deserialize the line {}! {:?}", line, err),
            },
            Err(err) => eprintln!("Failed to read the line {}! {:?}", index, err),
        }
        thread::sleep(Duration::from_millis(1000));
    }

    println!("The message file processing is complete.");
}
