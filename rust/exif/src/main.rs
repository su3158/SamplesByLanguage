extern crate exif;

use std::env;
use std::fs::File;
use std::io::BufReader;

use exif::{DateTime, In, Reader, Tag, Value};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 2 {
        println!("Usage: exif <image-file>");
        return;
    }

    let file = File::open(&args[1]).expect("failed to open file");
    println!("{:?}", file);
    // let mut reader = BufReader::new(&file);
    // println!("{:?}", reader);

    let exif = Reader::new()
        .read_from_container(&mut BufReader::new(&file))
        .unwrap();

    for field in exif.fields() {
        println!("{}: {}", field.tag, field.display_value().with_unit(&exif));
    }

    // To get unsigned integer value(s) from either of BYTE, SHORT,
    // or LONG, `Value::get_uint` or `Value::iter_uint` can be used.
    if let Some(field) = exif.get_field(Tag::PixelXDimension, In::PRIMARY) {
        if let Some(width) = field.value.get_uint(0) {
            println!("Valid width of the image is {}.", width);
        }
    }

    // To convert a Rational or SRational to an f64, `Rational::to_f64`
    // or `SRational::to_f64` can be used.
    if let Some(field) = exif.get_field(Tag::XResolution, In::PRIMARY) {
        match field.value {
            Value::Rational(ref vec) if !vec.is_empty() => {
                println!("X resolution is {}.", vec[0].to_f64())
            }
            _ => {}
        }
    }

    // To parse a DateTime-like field, `DateTime::from_ascii` can be used.
    if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
        match field.value {
            Value::Ascii(ref vec) if !vec.is_empty() => {
                if let Ok(datetime) = DateTime::from_ascii(&vec[0]) {
                    println!("Year of DateTime is {}.", datetime.year);
                }
            }
            _ => {}
        }
    }

    // To obtain a string representation, `Value::display_as`
    // or `Field::display_value` can be used.  To display a value with its
    // unit, call `with_unit` on the return value of `Field::display_value`.
    // let tag_list = [
    //     Tag::ExifVersion,
    //     Tag::PixelXDimension,
    //     Tag::XResolution,
    //     Tag::ImageDescription,
    //     Tag::DateTime,
    // ];
    // for tag in tag_list {
    //     if let Some(field) = exif.get_field(tag, In::PRIMARY) {
    //         println!("{}: {}", field.tag, field.display_value().with_unit(&exif));
    //     }
    // }

    // let mut exif_reader = Reader::new();
    // exif_reader
    //     .read_from_container(&mut std::io::BufReader::new(&file))
    //     .unwrap();

    // if let Some(exif) = exif_reader.get_field(Tag::ExifIFD) {
    //     let sub_exif_reader = exif.get_sub_exif();
    //     if let Some(sub_exif) = sub_exif_reader.and_then(|exif| exif.get()) {
    //         if let Ok(field) = sub_exif.get_field(Tag::DateTimeOriginal, In::UNDEFINED) {
    //             if let Ok(value) = sub_exif.get_field_value(&field) {
    //                 if let Some(date_time) = value.get_datetime() {
    //                     println!("Date Taken: {}", date_time.to_naive_local());
    //                 }
    //             }
    //         }
    //     }
    // }

    // let exif_reader = Reader::new()
    //     .read_from_container(&mut reader)
    //     .expect("failed to create exif reader");
    // println!("exif_reader: {:#?}", exif_reader);

    // if let Some(exif) = exif_reader.get_exif().and_then(|exif| exif.get()) {
    //     if let Ok(field) = exif.get_field(Tag::DateTimeOriginal, In::Unknown) {
    //         if let Ok(value) = exif_reader.get_field_value(&field) {
    //             if let Some(dt) = value.get_date_time() {
    //                 println!("DateTimeOriginal: {}", dt);
    //             }
    //         }
    //     }
    // }
}
