use worker::*;

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    Response::from_html(
        r#"
    <html>
        <head>
            <title>Hello, World!</title>
        </head>
        <body>
            <h1>Hello, World!</h1>
            <p>Hello, World! 2</p>
            <p>Hello, World! 12</p>
        </body>
    </html>
    "#,
    )
}
