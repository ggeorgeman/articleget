mod requests;
use requests::*;
use reqwest::Proxy;


#[tokio::main]
async fn main() {
    let proxy = Proxy::http("198.27.115.215").unwrap();
    let client = Client::builder().proxy(proxy).build().unwrap();
    for i in 1..101 {
        let l = ArticleLink{ 
            url: format!("https://habr.com/en/articles/{}/", i), 
            client: &client
        };
        println!("{:?}", l.content().await.unwrap());
    }
}

