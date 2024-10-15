use scraper::{ElementRef, Html, Selector};
use types::{Node, Type};

pub mod types;

pub fn parse(data: String) -> Vec<Node> {
    let document = Html::parse_document(&data);
    let selector = Selector::parse("body").unwrap();
    let html = document.select(&selector).next().unwrap();
    let links = document.select(&Selector::parse("a").unwrap())
        .map(|link: ElementRef<'_>| Node{ 
            content: link.value().attr("href").unwrap_or("").to_string(), 
            t: Type::A,
            text: link.inner_html()}
        ).collect::<Vec<Node>>();
    let text = html.text().into_iter().collect::<Vec<_>>();
    text.join("");
    links
}

fn handle_link(url: String){

}