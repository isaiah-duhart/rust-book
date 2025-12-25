use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let tile_fut_1 = page_title(&args[1]);
        let tile_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::race(tile_fut_1, tile_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right
            };
        
        println!("{url} completed first!");
        match maybe_title {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("No title was found")
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;

    (url, Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html()))
}