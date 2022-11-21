// fn main() {
//     let response = reqwest::blocking::get(
//         "https://crypto.com/price",
//     )
//     .unwrap()
//     .text()
//     .unwrap();

//     let document = scraper::Html::parse_document(&response);
//     //variables for scraping the data
//     let title_selector = scraper::Selector::parse("span.chakra-text.css-1jj7b1a").unwrap();
//     let title_selector1 = scraper::Selector::parse("td.css-w6jew4").unwrap();
//     let title_selector2=scraper::Selector::parse("p.chakra-text.css-dg4gux").unwrap();
//     let title_selector3=scraper::Selector::parse("div.css-0").unwrap();
//     let title_selector4=scraper::Selector::parse("td.css-1nh9lk8").unwrap();


//     //variables for storing the data
//     let pos = document.select(&title_selector).map(|x| x.inner_html());
//     let code=document.select(&title_selector1).map( |x| x.inner_html());
//     let price_per=document.select(&title_selector2).map(|x| x.inner_html());
//     let price=document.select(&title_selector3).map(|x| x.inner_html());
//     let volume=document.select(&title_selector4).map(|x| x.inner_html());
//     pos
//         .zip(1..51)
//         .for_each(|(item, number)| println!("{}. {}", number, item));
//     code
//         .zip(1..51)
//         .for_each(|(item, number)| println!("{}. {}", number, item));
//     price_per
//         .zip(1..51)
//         .for_each(|(item, number)| println!("{}. {}", number, item));
//     price
//         .zip(1..51)
//         .for_each(|(item, number)| println!("{}. {}", number, item));
//     volume
//         .zip(1..101)
//         .for_each(|(item,number)| println!("{}. {}",number,item));
    
//         // let mut n=1
//         // while n<101{
//         //     (|(item,number)| println!("{}. {}",number,item));
//         //     n+=1;
//         // }

    
//     }

