use async_trait::async_trait;
pub use reqwest::Client;
pub use anyhow::Result;
pub use scraper::{Html, Selector};


fn p_is_not_wikipedia_link(p: &str) -> bool {
    !p.starts_with('[') && !p.ends_with(']')
}

fn p_is_not_image_tag(p: &str) -> bool {
    !p.starts_with(".mw-") && !p.starts_with(".ts-")
}


#[async_trait]
pub trait Link {
    async fn content(&self) -> Result<String>;
}


pub struct ArticleLink<'a> {
    pub url: String,
    pub client: &'a Client,
}

impl ArticleLink<'_> {
    async fn page(&self) -> Result<Html> {
        let response = self.client.get(self.url.clone()).send().await?;
        Ok(Html::parse_document(&response.text().await?))
    }

    fn paragraphs(page: Html) -> Vec<String> {
        page.select(&Selector::parse("p").unwrap()).map(|p| {
            p.text()
                .map(String::from)
                .filter(|p| p_is_not_wikipedia_link(p) && p_is_not_image_tag(p))
                .collect::<Vec<String>>().join("")
        }).collect()
    }
}

#[async_trait]
impl<'a> Link for ArticleLink<'a> {
    async fn content(&self) -> Result<String> {
        let page = self.page().await?;
        let paragraphs = ArticleLink::paragraphs(page);
        Ok(paragraphs.join(" "))
    } 
}

