use reqwest;
use reqwest::StatusCode;
use scraper::{Html, Selector, ElementRef};

struct FetchResult {
    status: StatusCode,
    content: String,
}

async fn fetch(url: &str) -> Result<FetchResult, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;

    Ok(FetchResult {
        status: resp.status(),
        content: resp.text().await?,
    })
}

fn extract<'a,  T>(html: &'a Html, sel: &'a Selector, extractor: &'a T) -> String
    where T: Fn(&ElementRef) -> String
{
    match html.select(&sel).next() {
        Some(el) => extractor(&el),
        None => String::new(),
    }
}

fn extract_title_and_description(doc: &str) -> (String, String) {
    let html = Html::parse_document(doc);

    let title_sel = Selector::parse("head > title").unwrap();
    let desc_sel = Selector::parse("head > meta[name='description']").unwrap();

    let title_extractor = |el: &ElementRef| String::from(el.text().next().unwrap_or(""));
    let desc_extractor = |el: &ElementRef| String::from(el.value().attr("content").unwrap_or(""));

    let title = extract(&html, &title_sel, &title_extractor);
    let desc = extract(&html, &desc_sel, &desc_extractor);

    (title, desc)
}

#[cfg(test)]
mod testing {
    use crate::{fetch, extract_title_and_description};

    #[tokio::test]
    async fn fetch_url() {

        let result = fetch("https://rust-lang.org").await;
        assert!(result.is_ok());

        let (title, desc) = extract_title_and_description(&result.unwrap().content);

        assert_eq!("Rust Programming Language", title.trim());
        assert_eq!("A language empowering everyone to build reliable and efficient software.", desc.trim());
    }
}