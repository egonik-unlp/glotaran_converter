#![feature(path_file_prefix)]

use regex::Regex;

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::Write,
    path::Path, cell::RefCell,
    rc::Rc
};

const SOURCE: &'static str = "TRES_plaintext.txt";

fn main() {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(SOURCE)
        .expect("Problema leyendo archivo");
    let re = Regex::new(r"(\d){3}").unwrap();
    let headers = rdr
        .headers()
        .unwrap()
        .into_iter()
        .map(|rec| match re.captures(rec) {
            None => "canales",
            Some(caps) => caps.get(0).map_or("0", |m| m.as_str()),
        })
        .collect::<Vec<&str>>();
    println!("{:?}", headers);
    let mut body: Vec<Vec<_>> = vec![];
    for (n, record) in rdr.records().enumerate() {
        let mut line = record
            .unwrap()
            .into_iter()
            .map(|recn| recn.to_string())
            .collect::<Vec<String>>();
        line.insert(0, format!("{}", n ));
        body.push(line);
    }
    let headlines = headers.len();
    write_to_file(headers, body, headlines).expect("Problema escribiendo el archivo.")
}

fn write_to_file(headers: Vec<&str>,body: Vec<Vec<String>>, line_number: usize) -> Result<(), Box<dyn Error>> {
    let extensionless = Path::new(SOURCE).file_prefix().unwrap().to_str().unwrap();
    let filename = format!("{}.ascii", extensionless);
    let mut file = File::create(&filename)?;
    file.write(
        format!("{} \nwavelength explicit\nintervalnr {}", SOURCE, line_number).as_bytes()
    ).unwrap();
    file.flush().unwrap();
    let file2 = OpenOptions::new().append(true).open(&filename)?;
    let mut writer = csv::Writer::from_writer(file2);
    writer.write_record(&headers).unwrap();
    body.into_iter().for_each(|v| {writer.write_record(&v).unwrap()});
    Ok(())
}
