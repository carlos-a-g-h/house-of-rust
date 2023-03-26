use actix_web::App;
use actix_files::Files;

let app = App::new()
    .service(Files::new("/static", ".").prefer_utf8(true));
