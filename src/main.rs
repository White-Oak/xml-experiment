use quick_xml::events::Event;
use quick_xml::{Reader, Writer};
use zip::ZipWriter;

use std::fs::File;

fn main() {
    let mut reader = Reader::from_file("0.xml").unwrap();

    let write = File::create("/tmp/foo.xml").unwrap();
    let mut zip = ZipWriter::new(write);
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    zip.start_file("0.xml", options).unwrap();
    let mut writer = Writer::new_with_indent(zip, b' ', 2);

    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut in_offer = false;
    let now = std::time::Instant::now();
    let mut attempts = 0;

    loop {
        let event = reader.read_event(&mut buf);
        match event {
            Ok(event) => {
                match event {
                    Event::Start(ref e) if e.name() == b"offer" => {
                        in_offer = true;
                        writer.write_event(event).unwrap();
                    }

                    Event::End(ref e) if e.name() == b"offer" => {
                        in_offer = false;
                        writer.write_event(event).unwrap();
                        attempts += 1;
                        if attempts > 10000 {
                            break;
                        }
                    }

                    Event::Empty(_) => {
                        if in_offer {
                            writer.write_event(event).unwrap();
                        }
                    }

                    Event::Text(_) | Event::CData(_) => {
                        if in_offer {
                            writer.write_event(event).unwrap();
                        }
                    }

                    Event::Start(ref e) if e.name() != b"offer" => {
                        if in_offer {
                            writer.write_event(event).unwrap();
                        }
                    }

                    Event::End(ref e) if e.name() != b"offer" => {
                        if in_offer {
                            writer.write_event(event).unwrap();
                        }
                    }

                    Event::Eof => break, // exits the loop when reaching end of file

                    _ => (), // There are several other `Event`s we do not consider here
                }
            }

            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
        buf.clear();
    }
    let elapsed = now.elapsed().as_nanos();
    println!("Elapsed {} ns", elapsed);
}
