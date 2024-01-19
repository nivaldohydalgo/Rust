use actix_web::*;

#[actix_web::main]
async fn main() {
    println!("RUST Api demo...!");

    let api = HttpServer::new( || {
        App::new()
        .route("/ping", web::get().to(ping));
    });

    let porta = 9091;    
    let api = api.bind(format!("127.0.0.1:{}", porta))
    .expect("Não foi possível conectar...");

    println!("conectado com sucesso! \n http://localhost:{}", porta);

    api.run()
    .await
}

async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("conectado...");
}
