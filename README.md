# PCAP Window Splitter

This tool splits a PCAP file into separate files based on a time window, so each file would have several 
packets that are within a certain time window.

For example, for time window of 0.1 seconds and the next packets:
```
<number> <timestamp>
1. 0.0 
2. 0.05
3. 0.1
4. 0.15
5. 0.16
6. 0.17
7. 0.25
```

The result would be:
```
File 1:
1. 0.0
2. 0.05

File 2:
1. 0.1
2. 0.15
3. 0.16
4. 0.17

File 3:
1. 0.25
```

## Usage
```shell
pcap_window_splitter <pcap_filename> <output_folder> <time_window_seconds>
```

Example:
```shell
pcap_window_splitter test.pcap /tmp/output 0.1
```

## Compilation

Assuming you have Rust installed, you can compile the tool with:
```shell
cargo build --release
```

The resulting binary will be in `target/release/pcap_window_splitter`.

