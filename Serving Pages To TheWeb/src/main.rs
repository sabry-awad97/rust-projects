use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
            </form>
        "#,
    )
}

#[post("/gcd")]
async fn post_gcd(req_body: web::Form<GcdParameters>) -> impl Responder {
    if req_body.n == 0 || req_body.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        req_body.n,
        req_body.m,
        gcd(req_body.n, req_body.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(get_index).service(post_gcd));

    println!("Serving on http://127.0.0.1:3000...");

    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
}
