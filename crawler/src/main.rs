fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://niecke-it.de");

    println!("{:#?}", resp);
    Ok(())
}
