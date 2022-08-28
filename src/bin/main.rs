use manager::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("Err: {:?}", e);
    }

    let a = ["1", "two", "NaN", "four", "5"];

    let mut iter = a.iter().filter_map(|s| s.parse().ok());

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);
}
