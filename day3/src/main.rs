extern crate csv;
use csv::Writer;

fn main() {
    let dollar_films = vec![
        ("A Fistful of Dollars", "Rojo", 1964),
        ("For a Few Dollars More", "El Indio", 1965),
        ("The Good, the Bad and the Ugly", "Tuco", 1966),
    ];
    let path = "westerns.csv";
    let mut writer = Writer::from_file(path).unwrap();
    for row in dollar_films {
        writer.encode(row).expect("CSV writer error");
    }

}
