extern crate reqwest;

fn get_wrapper() -> Result<(), reqwest::Error> {
    let text = reqwest::get("https://www.leboncoin.fr")?
        .text()?;
    
    println!("body = {:?}", text);
    Ok(())
}
fn main() {
    get_wrapper().unwrap();
}
