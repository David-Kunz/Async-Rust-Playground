use {
    dotenv::dotenv,
    hyper::{
        service::{make_service_fn, service_fn},
        Body,
        Client,
        Request,
        Response,
        Server,
        Uri,
    },
    std::env,
    std::collections::HashMap,
    std::net::SocketAddr,
    std::str::FromStr
};

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("{}", req.uri());
    let url_str = "http://www.google.com";
    let url = url_str.parse::<Uri>().expect("failed to parse url");
    let res = Client::new().get(url).await?;
    Ok(res)
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);
    let serve_future = Server::bind(&addr)
        .serve(make_service_fn(|_| {
            async {
                {
                    Ok::<_, hyper::Error>(service_fn(serve_req))
                }
            }
        }));

    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let PORT = env::var("PORT")
        .unwrap_or("4004".to_string())
        .parse::<u16>()
        .expect("Port must be a valid integer");
    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    run_server(addr).await;
}
