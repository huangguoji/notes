use actix_web::{HttpServer, App, web, middleware, HttpRequest,Result};
use std::{io, env};
use actix_files;
use actix_files::NamedFile;
use notes::routes;
use env_logger::Env;
#[actix_rt::main]
async fn main()-> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(||{
        App::new()
            .wrap(middleware::Logger::default())
            .route("/",web::get().to(index))
            .configure(routes::all)
            .service(actix_files::Files::new("/static","."))

    }).bind("0.0.0.0:8000")?
    .run()
    .await
}


async fn index(_req:HttpRequest)-> Result<NamedFile>{
    Ok(NamedFile::open("./views/index.html")?)
}