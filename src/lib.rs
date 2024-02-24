use maud::{html, Markup, DOCTYPE};
use worker::*;

fn main_page() -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "Hello, Maud!" }
            link rel="stylesheet" type="text/css" href="./out.css";
        }
        body {
            h1 .text-2xl { "Hello, Maud-Title!" }
            p .italic { "Hello, World! 1" }
            p { "Hello, World! 2" }
        }
    }
}

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    Response::from_html(main_page().into_string())
}
