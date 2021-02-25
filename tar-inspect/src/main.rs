fn main() {
    if std::env::args().len() < 2 {
        eprintln!("Please provide tar file name to review.");
        return;
    }

    let filename = std::env::args().into_iter().nth(1).unwrap();

    let path = std::path::Path::new(filename.as_str());
    if !path.exists() {
        eprintln!("The given path: '{}' does not exist.", filename);
        return;
    }

    let file_reader = std::fs::File::open(path).unwrap();
    let buf_reader = std::io::BufReader::new(file_reader);
    let gz_reader = flate2::bufread::GzDecoder::new(buf_reader);
    let mut tar_reader = tar::Archive::new(gz_reader);

    for entry in tar_reader.entries().unwrap() {
        println!("{:#?}", entry.unwrap().header());
    }
}
