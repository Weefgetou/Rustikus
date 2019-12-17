/*use actix_web::{web, App, HttpServer, Responder};

fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new().service(
              web::resource("/{id}/{name}/index.html").to(index)))
        .bind("127.0.0.1:8080")?
        .run()
}
*/

use rodio::Sink;

use std::{thread, time};



fn main(){
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    // Add a dummy source of the sake of the example.
    let source = rodio::source::SineWave::new(440);
    sink.append(source);
    let thousand_millis = time::Duration::from_millis(1000);

    thread::sleep(thousand_millis);

}
