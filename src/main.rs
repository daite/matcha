#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};
use prettytable::format;
use reqwest::header::USER_AGENT;
use select::document::Document;
use select::predicate::{Class, Name};
use std::sync::{Arc, Mutex};
use clap::{arg, Command};
use urlencoding::encode;
use reqwest;
use tokio;

const MY_USER_AGENT: &str = concat!(
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) ", 
    "AppleWebKit/537.36 (KHTML, like Gecko) ",
    "Chrome/98.0.4758.109 Safari/537.36"
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let matches = Command::new("Matcha Latte")
    .version("0.1.0")
    .author("daite <daite@gmail.com>")
    .about("Matcha Latte!")
    .arg(arg!(-k --keyword <KEYWORD>).required(true))
    .get_matches();
   let keyword = matches.value_of("keyword").expect("required");
   println!("[*] Searching torrents for {}", keyword);
   execute(keyword).await
}

async fn get_magnet(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get(url)
                .header(USER_AGENT, MY_USER_AGENT)
                .send()
                .await?
                .text()
                .await?;
    let document = Document::from(&body[..]);
    let  mut magnet = "";
    for node in document.find(Class("list-group-item")){
        if let Some(m) = node.find(Name("a")).next() {
            magnet = m.attr("href").unwrap();
        }
    }
    Ok(magnet.to_string())
}

async fn execute(keyword: &str) -> Result<(), Box<dyn std::error::Error>> {
    let site_url = "https://torrentjuju.com/bbs/";
    let url = format!(
        "{}{}{}",
        site_url,
        "search.php?search.php&stx=",
        encode(keyword)
    );
    let client = reqwest::Client::new();
    let body = client.get(url)
                .header(USER_AGENT, MY_USER_AGENT)
                .send()
                .await?
                .text()
                .await?;
    let document = Document::from(&body[..]);
    let counter = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for node in document.find(Class("media-heading")){
        let title = node.text().replace("\n", "");
        let bbs_link = 
            node.find(Name("a"))
                .next()
                .unwrap()
                .attr("href")
                .unwrap();
        let bbs_link = format!("{}{}", site_url, bbs_link.replace("./", ""));
        let counter = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
           if let Ok(magnet) = get_magnet(&bbs_link[..]).await {
            let mut m = counter.lock().unwrap();
            m.push((title, magnet));
           }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await?;
    }
    let mut table = Table::new();
    table.set_titles(row!["Title", "Magnet"]);
    for (title, magnet) in &*counter.lock().unwrap() {
        table.add_row(Row::new(
            vec![
                Cell::new(&title),
                Cell::new(&magnet),
            ]
        ));
    }
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
    Ok(())
}