use prettytable::Table;
use prettytable::row;


    fn main() {
        let mut table=Table::new();
        let response = reqwest::blocking::get(
            "https://crypto.com/price",
        )
        .unwrap()
        .text()
        .unwrap();
    
        let document = scraper::Html::parse_document(&response);
        //variables for scraping the data
        let title_selector = scraper::Selector::parse("span.chakra-text.css-1jj7b1a").unwrap();//position
        let title_selector1 = scraper::Selector::parse("td.css-w6jew4").unwrap();//code of the cryto
        let title_selector2=scraper::Selector::parse("p.chakra-text.css-dg4gux").unwrap();//price percentage
        let title_selector3=scraper::Selector::parse("div.css-b1ilzc").unwrap();//price current
        let title_selector4=scraper::Selector::parse("td.css-1nh9lk8").unwrap();//24h volume
        // let title_selector4=scraper::Selector::parse("td.css-1nh9lk8").unwrap();
    
    
        //variables for storing the data
        let mut code = document.select(&title_selector).map(|x| x.inner_html());
        let pos=document.select(&title_selector1).map( |x| x.inner_html());
        let mut price_per=document.select(&title_selector2).map(|x| x.inner_html());
        let price=document.select(&title_selector3).map(|x| x.inner_html());
        let volume=document.select(&title_selector4).map(|x| x.inner_html());
        
       for i in code{
            for j in price{
                table.add_row(row![{i},{j}]);
            }
       
        }

        table.printstd();}
       




    //   code
    
    //     .for_each(|item|{ table.add_row(row!({item})); });
    // price
            
    //     .for_each(|item|{ table.add_row(row!({item}));});
        

        
        // println!("\n|{}|{}|{}|\n",code,price,volume);
        // code
            
        //     .for_each(|item| println!("{}",item));
        

        // pos
            
        //     .for_each(|item| println!(" {}", item));
        // price_per
            
        //     .for_each(|item| println!("{}", item));
        // price
            
        //     .for_each(|item| println!("{}. {}", number, item));
        // volume
            
        //     .for_each(|item| println!("{}. {}",number,item));
        
            // let mut n=1
            // while n<101{
            //     (|(item,number)| println!("{}. {}",number,item));
            //     n+=1;
            // }
    
        
        
    
