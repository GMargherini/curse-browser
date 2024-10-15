use reqwest;
use cursive::views::{Button, EditView, LinearLayout, Panel, ScrollView, TextView};
mod html;
mod css;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.
    // Parse our URL...
    let url = "https://en.wikipedia.org/wiki/Linux".parse::<hyper::Uri>()?;

    // Get the host and the port
    let protocol = url.scheme_str().unwrap_or("http");
    let host = url.host().expect("uri has no host");
    
    let port: u16 = match protocol {
        "http" => 80,
        "https" => 443,
        _ => url.port_u16().unwrap_or(80)
    };

    let address = format!("{}:{}", host, port);
    println!("{}", address);

    let res = reqwest::get(url.to_string()).await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;

    let document = html::parse(body);
    let data = document.iter().map(|node| node.text.to_owned()).collect::<Vec<_>>().join("");
    
    let mut siv = cursive::default();

	siv.add_global_callback('q', |s| s.quit());
    let buttons = LinearLayout::vertical().child(Button::new(document[0].text.to_owned(), |_| ()));

    
	siv.add_fullscreen_layer(Panel::new(
        LinearLayout::vertical()
                .child(LinearLayout::horizontal().child(EditView::new()).child(Button::new("Go", |_|())))
                .child(ScrollView::new(buttons)))
        .title(url.to_string()));
// TextView::new(data)
	siv.run();

    Ok(())
}
