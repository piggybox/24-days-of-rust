extern crate csv;
extern crate rustc_serialize;

use csv::Reader;
use csv::Writer;

fn main() {
    #[derive(RustcDecodable, RustcEncodable)]
    struct Movie {
        title: String,
        bad_guy: String,
        pub_year: usize,
    }

    let movie = Movie {
        title: "Hang 'Em High".to_string(),
        bad_guy: "Wilson".to_string(),
        pub_year: 1968,
    };

    let path = "westerns.csv";
    let mut writer = Writer::from_file(path).unwrap();
    writer.encode(movie).expect("CSV writer error");
    writer.flush().expect("Flush error");

    let mut reader = Reader::from_file(path).unwrap().has_headers(false);
    for row in reader.decode() {
        let movie: Movie = row.unwrap();
        println!(
            "{} was a bad guy in '{}' in {}",
            movie.bad_guy, movie.title, movie.pub_year
        );
    }
}
