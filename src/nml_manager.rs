use std::io::Cursor;
use std::borrow::Cow;
use std::collections::HashMap;

use quick_xml::Writer;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesEnd, BytesStart};

use fileaccessor;
use track::Track;
use TRACKS_TARGET_FOLDER;
use NML_LOCATION;

use DebugMode;
use DEBUG_MODE;

pub fn run(track_list: &Vec<Track>) {
    match track_list.is_empty() {
        false => {
            let mut reader = Reader::from_file(NML_LOCATION).unwrap();
            let nml_formatted_dir = TRACKS_TARGET_FOLDER.replace("/", "/:");

            let mut file_mappings: HashMap<String, Track> = HashMap::new();
            for track in track_list {
                file_mappings.insert(format!("{}.{}", naive_nml_encode(&track.file_name.clone()), track.extension.clone()), track.clone());
            };

            match DEBUG_MODE {
                DebugMode::NML | DebugMode::ALL => {
                    println!("nml_manager: printing org entries which need to be kept up to date with NML");
                    println!("nml_manager: nml_formatted_dir {}", nml_formatted_dir);
                    println!("{:#?}", track_list);
                },
                _ => ()
            }

            //removes the identation of the file if set to true
            reader.trim_text(false);

            let mut writer = Writer::new(Cursor::new(Vec::new()));
            let mut buf = Vec::new();

            let mut in_collection = false;

            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) if !in_collection && e.name() == b"PRIMARYKEY" => {
                        let mut elem = BytesStart::owned(b"PRIMARYKEY".to_vec(), "PRIMARYKEY".len());

                        elem.extend_attributes(e.attributes().map(|attr| {
                            let mut attr = attr.unwrap();

                            if attr.key == b"KEY" {
                                let original_value = String::from_utf8(attr.value.as_ref().to_owned()).unwrap();
                                let value_splitted = original_value.split("/:").collect::<Vec<&str>>();
                                let original_name = value_splitted.last().unwrap();

                                if file_mappings.contains_key(original_name.to_owned()) {
                                    let matching_track = &file_mappings.get(original_name.to_owned()).unwrap();
                                    attr.value = Cow::from(format!("{}{}/:{}/:{}",
                                        value_splitted[0],
                                        nml_formatted_dir,
                                        matching_track.release_year,
                                        naive_nml_encode(&matching_track.short_name))
                                        .as_bytes().to_vec());
                                }
                            }

                            attr
                        }));

                        assert!(writer.write_event(Event::Start(elem)).is_ok());
                    },
                    Ok(Event::End(ref e)) if !in_collection && e.name() == b"PRIMARYKEY" => {
                        assert!(writer.write_event(Event::End(BytesEnd::borrowed(b"PRIMARYKEY"))).is_ok());
                    },
                    Ok(Event::Start(ref e)) if in_collection && e.name() == b"ENTRY" => {
                        let mut entry_elem = BytesStart::owned(b"ENTRY".to_vec(), "ENTRY".len());

                        let mut inner_buf = Vec::new();

                        loop {
                            match reader.read_event(&mut inner_buf) {
                                Ok(Event::Start(ref e_inner)) if e_inner.name() == b"LOCATION" => {
                                    let mut location_elem = BytesStart::owned(b"LOCATION".to_vec(), "LOCATION".len());

                                    let file_name = e_inner.attributes()
                                        .filter_map(|a| a.ok())
                                        .find(|a| a.key == b"FILE");

                                    let file_name = String::from_utf8(file_name.unwrap().value.as_ref().to_owned()).unwrap();

                                    if file_mappings.contains_key(&file_name) {
                                        let matching_track = file_mappings.get(&file_name).unwrap();

                                        location_elem.extend_attributes(e_inner.attributes().map(|attr| {
                                            let mut attr = attr.unwrap();

                                            match attr.key {
                                                b"FILE" => attr.value = Cow::from(naive_nml_encode(&matching_track.short_name).as_bytes().to_vec()),
                                                b"DIR" => attr.value = Cow::from(
                                                        format!("{}/:{}", nml_formatted_dir, matching_track.release_year).as_bytes().to_vec()),
                                                _ => ()
                                            }

                                            attr
                                        }));
                                        entry_elem.extend_attributes(e.attributes().map(|attr| {
                                            let mut attr = attr.unwrap();

                                            match attr.key {
                                                b"TITLE" => attr.value = Cow::from(naive_nml_encode(&matching_track.title).as_bytes().to_vec()),
                                                b"ARTIST" => attr.value = Cow::from(naive_nml_encode(&matching_track.author).as_bytes().to_vec()),
                                                _ => ()
                                            }

                                            attr
                                        }));
                                    } else {
                                        entry_elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                                        location_elem.extend_attributes(e_inner.attributes().map(|attr| attr.unwrap()));
                                    }

                                    assert!(writer.write_event(Event::Start(entry_elem.to_owned())).is_ok());
                                    assert!(writer.write_event(Event::Start(location_elem)).is_ok());
                                },
                                Ok(Event::End(ref e)) if e.name() == b"LOCATION" => {
                                    assert!(writer.write_event(Event::End(BytesEnd::borrowed(b"LOCATION"))).is_ok());
                                },
                                Ok(Event::End(ref e)) if e.name() == b"ENTRY" => {
                                    assert!(writer.write_event(Event::End(BytesEnd::borrowed(b"ENTRY"))).is_ok());
                                    break;
                                },
                                Ok(e) => assert!(writer.write_event(&e).is_ok()),
                                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                            }
                            inner_buf.clear();
                        }
                    },
                    Ok(Event::Start(ref e)) if e.name() == b"COLLECTION" => {
                        in_collection = true;
                        assert!(writer.write_event(Event::Start(e.to_owned())).is_ok());
                    },
                    Ok(Event::End(ref e)) if e.name() == b"COLLECTION" => {
                        in_collection = false;
                        assert!(writer.write_event(Event::End(BytesEnd::borrowed(b"COLLECTION"))).is_ok());
                    },
                    Ok(Event::Eof) => break,
            	    // you can use either `e` or `&e` if you don't want to move the event
                    Ok(e) => assert!(writer.write_event(&e).is_ok()),
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                }
                buf.clear();
            }

            fileaccessor::write_nml_file(writer.into_inner().into_inner());
            println!("nml_manager: Updated nml collection written to {}!", fileaccessor::get_nml_filename());
        },
        true => println!("nml_manager: Nothing to update!")
    }
}

fn naive_nml_encode(unescaped: &str) -> String {
    unescaped.replace("&", "&amp;").replace("\"", "&quot;").replace(":", "//")
}
