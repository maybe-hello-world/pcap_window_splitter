use std::error::Error;
use pcap::{Capture};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        println!("Usage: {} <pcap_filename> <output_folder> <time_window_seconds>", args[0]);
        println!("Example: {} test.pcap /tmp/output 0.1", args[0]);
        return Ok(());
    }

    let pcap_filepath: String = args[1].parse()?;
    let pcap_filename = pcap_filepath.split("/").last().and_then(|x| x.strip_suffix(".pcap")).unwrap();

    let output_folder: String = args[2].parse()?;

    let time_window_sec: f64 = args[3].parse()?;

    let mut cap = Capture::from_file(pcap_filepath.clone())?;
    let cap_datalink = cap.get_datalink().clone();
    let mut current_ts_window = 0.0;
    let mut window_counter = 0;

    let mut save_file = Capture::dead(cap_datalink)
        .unwrap()
        .savefile(format!("{}/{}_window{}.pcap", output_folder, pcap_filename, window_counter))
        .unwrap();

    while let Ok(packet) = cap.next_packet() {
        if current_ts_window == 0.0 {
            current_ts_window = packet.header.ts.tv_sec as f64;
            current_ts_window += packet.header.ts.tv_usec as f64 / 1_000_000.0;
        }

        let packet_ts = packet.header.ts.tv_sec as f64 + packet.header.ts.tv_usec as f64 / 1_000_000.0;
        if packet_ts > current_ts_window + time_window_sec {
            save_file.flush().unwrap();

            while packet_ts > current_ts_window + time_window_sec {
                current_ts_window += time_window_sec;
            }
            window_counter += 1;

            save_file = Capture::dead(cap_datalink)
                .unwrap()
                .savefile(format!("{}/{}_window{}.pcap", output_folder, pcap_filename, window_counter))
                .unwrap();
        }

        save_file.write(&packet);
    }
    Ok(())
}
