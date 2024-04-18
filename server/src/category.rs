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

use actix_web::{get, web, Result};
use maud::{html, Markup};
use titlecase::titlecase;

use std::fs::read_dir;

use crate::article::read_article;
use crate::template::{render_page, NamedUrl, Navigation};

#[rustfmt::skip::macros(html)]
#[get("/{category_name}/index.html")]
pub async fn category_page(path: web::Path<String>) -> Result<Markup> {
    let category_name = path.into_inner();
    let category_title = titlecase(&category_name);

    Ok(render_page(
        &category_title,
        &Navigation {
            page_title: category_title.clone(),
            parents: vec![NamedUrl {
                name: String::from("Encyclopedia of Precision Medicine"),
                url: String::from("/"),
            }],
        },
        html! {
	    div .narrow-content {
		section #overview {
		    h1 {
			(category_title)
		    }
		}
		(list_articles(category_name))
	    }
	},
    ))
}

#[rustfmt::skip::macros(html)]
pub fn list_articles(category_name: String) -> Markup {
    if let Ok(entries) = read_dir(format!("../articles/{}/", category_name)) {
        html! {
	    section #articles {
		ul {
		    @for entry in entries {
			@if let Ok(entry) = entry {
			    @if let Ok(article_name) = entry.file_name().into_string() {
				@if let Ok(article) = read_article(&category_name, &article_name) {
				    li {
					a href=(format!("/{}/{}/index.html", category_name, article_name)) {
					    (titlecase(&article.name))
					}
				    }
				}
			    }
			}
		    }
		}
	    }
        }
    } else {
        panic!("Could not find category.");
    }
}
