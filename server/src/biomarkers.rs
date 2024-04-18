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
use serde_json::de::from_reader;
use titlecase::titlecase;

use std::error::Error;
use std::fs::File;

use crate::models::Article;
use crate::template::{render_page, NamedUrl, Navigation};

#[rustfmt::skip::macros(html)]
#[get("/biomarkers/{biomarker_class}/index.html")]
pub async fn biomarker_page(path: web::Path<String>) -> Result<Markup> {
    let biomarker_class = path.into_inner();
    let biomarker_article = read_article(&biomarker_class);

    if let Ok(biomarker_article) = biomarker_article {
        Ok(render_page(
            &biomarker_article.name.clone(),
            &Navigation {
                page_title: titlecase(&biomarker_article.name.clone()),
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

fn read_article(biomarker_class: &String) -> Result<Article, Box<dyn Error>> {
    let file = File::open(format!(
        "../articles/biomarkers/{}/article.json",
        biomarker_class
    ))?;

    let article = from_reader(file)?;
    Ok(article)
}

#[rustfmt::skip::macros(html)]
fn markup_article(article: Article) -> Markup {
    html! {
	div .narrow-content {
	    h1 {
		(article.name)
	    }
	    @if let Some(stub_issue) = article.stub_issue {
		p {
		    "This article is currently a stub. That means that it exists and can be linked to, but it doesn't have any descriptive text yet!"
		}
		p {
		    "Would you be interested in contributing to this article? If yes, please check our "
			a href=(format!("https://github.com/translating-science/encyclopedia/issues/{}", stub_issue)) {
			    "issue tracker"
			}
		    " to learn more about how to contribute!"    
		}
	    }
	}
    }
}
