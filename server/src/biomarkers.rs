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

use actix_web::error::ErrorNotFound;
use actix_web::{get, web, Result};
use maud::{html, Markup};
use titlecase::titlecase;

use crate::article::{markup_article, read_article_markup};
use crate::template::{render_page, NamedUrl, Navigation};

#[rustfmt::skip::macros(html)]
#[get("/biomarkers/{biomarker_class}/index.html")]
pub async fn biomarker_page(path: web::Path<String>) -> Result<Markup> {
    let biomarker_class = path.into_inner();
    let biomarker_article = read_article_markup(&String::from("biomarkers"), &biomarker_class);

    if let Ok(biomarker_article) = biomarker_article {
        Ok(render_page(
            &biomarker_article.article.name.clone(),
            &Navigation {
                page_title: titlecase(&biomarker_article.article.name.clone()),
                parents: vec![
                    NamedUrl {
                        name: String::from("Encyclopedia of Precision Medicine"),
                        url: String::from("/"),
                    },
                    NamedUrl {
                        name: String::from("Biomarkers"),
                        url: String::from("/biomarkers/index.html"),
                    },
                ],
            },
            html! {
		div .full-width {
		    (markup_article(biomarker_article))
		}
	    },
        ))
    } else {
        Err(ErrorNotFound(format!(
            "Could not find page for biomarker {}",
            biomarker_class
        )))
    }
}
