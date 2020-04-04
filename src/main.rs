use {
    hyper::{
        service::{make_service_fn, service_fn},
        Body, Client, Request, Response, Server, Uri,
    },
    std::net::SocketAddr,
    serde_json
};
use serde::{Serialize, Deserialize};

mod config;
mod entity;

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("{}", req.uri());
    let employee = entity::Employee {
        name: "David Kunz".to_string(),
        birth_year: 1986,
        employment_type: entity::EmploymentType::Permanent,
    };
    let body = serde_json::to_string(&employee).unwrap();

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(body))
        .unwrap())
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);
    let serve_future = Server::bind(&addr).serve(make_service_fn(|_| async {
        {
            Ok::<_, hyper::Error>(service_fn(serve_req))
        }
    }));

    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
    let config = config::get_config().await;
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    run_server(addr).await;
}
