pub mod board_layout;

pub fn get_input<T>(message: &str) -> T
where
    T: TryFrom<String>,
{
    println!("{}", message);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut res = T::try_from(line);
    while res.is_err() {
        let mut line = String::new();
        println!("Invalid, try again. {message}");
        std::io::stdin().read_line(&mut line).unwrap();
        res = T::try_from(line);
    }
    res.ok().unwrap()
}
