/**
 * Copyright (c) 2023 Bluespada <pentingmain@gmail.com>
 * 
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 * 
 * ---
 * 
 * I Love modular monolith architecture, so we spare each service its own folder or file.
 * But by default, we use a single file for each service.
*/

use actix_web::{HttpResponse, Scope, web};

pub fn services() -> Scope {
    web::scope("")
        .route("/api", web::post().to(handle_api))
        .default_service(web::to(handle_spa))
}

pub async fn handle_spa() -> std::io::Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html")
            .body(r"
<html>
    <head>
        <title>Example</title>
    </head>
    <body>
        <h1>Example</h1>
    </body>
</html>
            ")
    )
}

pub async fn handle_api() -> std::io::Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
        .content_type("application/json")
        .json({})
    )
}