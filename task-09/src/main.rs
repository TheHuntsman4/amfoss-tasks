fn main() {
    let response = reqwest::blocking::get(
        "https://crypto.com/price",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);
    //variables for scraping the data
    let title_selector = scraper::Selector::parse("tbody.css-0").unwrap();

    let pos = document.select(&title_selector).map(|x| x.inner_html());

    pos
        .zip(1..51)
        .for_each(|(item, number)| println!("{}. {}", number, item));