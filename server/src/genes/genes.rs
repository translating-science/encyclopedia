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

use actix_web::{get, Result};
use maud::{html, Markup};

use std::fs::read_dir;

use crate::template::{render_page, NamedUrl, Navigation};

#[rustfmt::skip::macros(html)]
#[get("/genes/index.html")]
pub async fn genes() -> Result<Markup> {
    Ok(render_page(
        &String::from("Genes"),
        &Navigation {
            page_title: String::from("Genes"),
            parents: vec![NamedUrl {
                name: String::from("Encyclopedia of Precision Medicine"),
                url: String::from("/"),
            }],
        },
        html! {
	    div .narrow-content {
		section #overview {
		    h1 {
			"Genes"
		    }
		}
		(list_genes())
	    }
	},
    ))
}

#[rustfmt::skip::macros(html)]
pub fn list_genes() -> Markup {
    if let Ok(entries) = read_dir("../articles/genes/") {
        html! {
	    section #genes {
		ul {
		    @for entry in entries {
			@if let Ok(entry) = entry {
			    @if let Ok(gene) = entry.file_name().into_string() {
				li {
				    a href=(format!("/genes/{}/index.html", gene)) {
					(gene)
				    }
				}
			    }
			}
		    }
		}
	    }
        }
    } else {
        panic!("Could not find gene files.");
    }
}
