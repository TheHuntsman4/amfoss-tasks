use prettytable::Table;
use prettytable::csv::Writer;



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
        
        let title_selector0=scraper::Selector::parse("div.css-ttxvk0>p").unwrap(); //name
        let title_selector1= scraper::Selector::parse("span.chakra-text.css-1jj7b1a").unwrap();//code
        let price = scraper::Selector::parse("div.css-b1ilzc").unwrap();//price current 
        let title_selector3=scraper::Selector::parse("td.css-1nh9lk8").unwrap();//24h volume+market cpa
        let title_selector4=scraper::Selector::parse("p.chakra-text.css-dg4gux").unwrap();//price percentage

        //variables for storing the data
        let name=document.select(&title_selector0).map(|x| x.inner_html()); 
        let code=document.select(&title_selector1).map(|x| x.inner_html());
        let prices = document.select(&price).map(|x| x.inner_html());
        let volumemark=document.select(&title_selector3).map(|x| x.inner_html());
        // let price_per=document.select(&title_selector3).map(|x| x.inner_html());

        

        //vector variables to input into csv

        let mut code_csv:Vec<String>= vec![String::new();0];
        let mut title_csv:Vec<String>= vec![String::new();0];
        let mut cryprice: Vec<String> = vec![String::new(); 0];
        let mut volume_csv:Vec<String>=vec![String::new();0];
        let mut mark_cap_csv:Vec<String>=vec![String::new();0];

        //getting all values into vector 

        for i in name{title_csv.push(i);}
        for i in code{code_csv.push(i);}
        for i in prices{cryprice.push(i);}

        //seperating the volume and market cap
        let mut j=0;
        for i in volumemark{
            if j%2==0{
                mark_cap_csv.push(i);
            }
            else{
                volume_csv.push(i);
            }
            j=j+1   ;
        }

        //dumping into teh csv file 
        let mut wtr=Writer::from_path("cryptocargo.csv").unwrap();
        wtr.write_record(&["Name","Code","Price", "24hr Hour volume", "Market Cap"]).unwrap();
        for i in 0..50{
   
        wtr.write_record([&title_csv[i],&code_csv[i],&cryprice[i],&volume_csv[i],&mark_cap_csv[i]]);
        wtr.flush();
   }
    }
        
    
    
    
    
    
    
    
    
    
    
    
    
        //    for i in code{
    //         for j in price{
    //             table.add_row(row![{i},{j}]);
    //         }
       
    //     }

    //     table.printstd();}
       




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
    
        
        
    
