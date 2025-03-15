/**
 * Copyright (c) 2023 Bluespada <pentingmain@gmail.com>
 * 
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
*/
use actix_web::{App, HttpServer};

mod vite;
mod services;


#[tokio::main]
async fn main(){
    dotenv::dotenv().ok();
    let http_server = HttpServer::new(move || {
        App::new()
            .service(services::example::services())
    })
    .bind(("0.0.0.0", 3000))
    // throw an error if the port is already in use
    .expect("Failed to bin port")
    // I recommend to keep the workers default (using all cpu thread), but if you a facing
    // performance issues, you can set it to 1
    // .workers(1)
    .run();

    let _ = tokio::join!(
        http_server,
        // add more services if you want example, if you want
        // using multiple services with different port or something else
        // related with your project needs.
    );
}
