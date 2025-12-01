use {
    common::{
        message::SequenceId,
        serializer::{CsvSerializer, Serializer, BUFFER_SIZE},
        EOL,
    },
    converter::{convert_arbitrage, create_swap_file_name},
    env_logger,
    log::{error, info},
    std::{
        env,
        fs::File,
        io::{BufRead, BufReader, Write},
        path::Path,
        time::Instant,
    },
};

fn main() {
    println!("Starting the Converter.");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing the Arbitrage messages file startup argument!");
    }

    env_logger::init();

    let arb_file = &args[1];
    info!("Opening the Arbitrage message file ({arb_file}).");
    let arb_file_path = Path::new(arb_file);
    let arb_file_reader = BufReader::new(File::open(arb_file_path).unwrap());

    let swap_file_name = create_swap_file_name();
    info!("Creating the Swap message file ({:?})", swap_file_name);
    let mut swap_file_handle = File::create(swap_file_name).unwrap();

    let mut serializer = CsvSerializer::new();
    let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];

    let mut seq_id = SequenceId::new();
    let mut counter = 0;
    let start = Instant::now();
    for line in arb_file_reader.lines() {
        let arb_str = line.unwrap();
        match serializer.deserialize_message(arb_str.as_bytes()) {
            Ok(msg) => match msg {
                common::message::Message::Arbitrage(arb) => {
                    for swap in convert_arbitrage(&mut seq_id, &arb) {
                        let size = serializer.serialize_swap(&swap, &mut buffer).unwrap();
                        swap_file_handle.write_all(&buffer[0..size]).unwrap();
                        swap_file_handle.write_all(&[EOL]).unwrap();
                    }
                    counter += 1;
                }
                _ => error!("Discarding an unexpected message ({})!", msg),
            },
            Err(error) => error! {"Failed to deserialize Arbitrage message ({})! {:?}", arb_str, error},
        }
    }
    println!(
        "The Converter completed. Processed {} messages in {} ms.",
        counter,
        start.elapsed().as_millis(),
    );
}
