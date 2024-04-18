// Licensed to Translating Science PBC under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  Translating Science PBC licenses
// this file to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use actix_files as fs;
use actix_web::{web, App, HttpServer};

use ts_encyclopedia::biomarkers::biomarker_page;
use ts_encyclopedia::evidence::evidence_class_page;
use ts_encyclopedia::genes::gene::gene_page;
use ts_encyclopedia::genes::genes::genes;
use ts_encyclopedia::landing::landing_page;
use ts_encyclopedia::therapies::therapy::therapy_page;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/resources", "./resources").show_files_listing())
            .service(web::redirect("/", "/index.html"))
            .service(landing_page)
            .service(biomarker_page)
            .service(evidence_class_page)
            .service(gene_page)
            .service(genes)
            .service(therapy_page)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
