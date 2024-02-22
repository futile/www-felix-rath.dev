use maud::{html, Markup, DOCTYPE};
use worker::*;

fn main_page() -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "Hello, Maud!" }
        }
        body {
            h1 { "Hello, Maud-Title!" }
            p { "Hello, World! 2" }
            p { "Hello, World! 22" }
        }
    }
}

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    Response::from_html(main_page().into_string())
}
