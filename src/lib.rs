//? easily read anything from the command line then save it to any variable
// let a: String = yoy_read();
pub fn yoy_read() -> String {
    let mut reader = String::new();
    let _ = std::io::stdin().read_line(&mut reader);
    return reader;
}