use brlapi::*;
fn main() -> Result<(), std::str::Utf8Error> {
    let brlapi = BrlAPI::new();
    let (_fd, auth, host) = brlapi.open_connection(
        None,
        Some(":1")
    );
    println!("{}, {}", auth, host);
    println!(
        "{} {}",
        brlapi.get_driver_name().unwrap(),
        brlapi.get_model_identifier().unwrap()
    );
    Ok(())
}
