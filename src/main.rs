use reqwest;
use tokio;
use colored::*;
async fn get_and_parse_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get(url).await?;
    let version = format!("{:?}", res.version());
    println!("{} {} {}", format!("{:?}", res.version()).purple(), res.status().as_str().purple(),res.status().canonical_reason().unwrap_or("unknown").blue());
    for (key,value) in res.headers().iter() {
        println!("{}: {}",key.to_string().blue(),value.to_str().unwrap_or("unknown").black());
    }
    println!();
    let body = res.text().await?;
    let mut in_tag = false;
    for c in body.chars() {
        if c == '<' {
            in_tag = true;
            print!("{}",c.to_string().purple());
        } else if c == '>' {
            in_tag = false;
            print!("{}",c.to_string().purple());
        } else if in_tag {
            print!("{}",c.to_string().purple());
        } else {
            print!("{}",c.to_string().black());
        }
    }
    Ok(())
}
#[tokio::main]
async fn main() {
    // let url = std::env::args().nth(1).expect("please provide a URL");
    //use args[1] as url or https://www.rust-lang.org/ as default 
    let url = std::env::args().nth(1).unwrap_or({
        println!("use https://www.rust-lang.org/ as default url");
        "https://www.rust-lang.org/".to_string()});
    get_and_parse_url(&url).await.unwrap();
}
