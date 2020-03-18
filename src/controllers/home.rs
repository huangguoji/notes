use actix_web::{web,get,HttpRequest,HttpResponse,Responder};

#[get("")]
pub async fn index()->impl Responder{
    format!("helo world")
}

#[get("/test")]
pub async fn test()->impl Responder{
    format!("helo world test")
}


pub fn default(config:&mut web::ServiceConfig){
    config
        .service(index)
        .service(test)
    ;
}