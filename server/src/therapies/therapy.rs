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
use std::io::Error as IoError;

use crate::markdown::{direct_to_markup, file_to_markup, ReferencedMarkup};
use crate::models::Therapy;
use crate::reference::generate_reference_list;
use crate::template::{render_page, NamedUrl, Navigation};

#[rustfmt::skip::macros(html)]
#[get("/therapies/{therapy_class}/{therapy_name}/index.html")]
pub async fn therapy_page(path: web::Path<(String, String)>) -> Result<Markup> {
    let (therapy_class, therapy_name) = path.into_inner();
    let therapy_result = read_therapy(&therapy_class, &therapy_name);
    let therapy_markdown = read_therapy_markdown(&therapy_class, &therapy_name);

    if let (Ok(therapy), Ok(mut therapy_markdown)) = (therapy_result, therapy_markdown) {
        Ok(render_page(
            &therapy_name,
            &Navigation {
                page_title: titlecase(&therapy_name),
                parents: vec![
                    NamedUrl {
                        name: String::from("Encyclopedia of Precision Medicine"),
                        url: String::from("/"),
                    },
                    NamedUrl {
                        name: String::from("Therapies"),
                        url: String::from("/therapies/index.html"),
                    },
                    NamedUrl {
                        name: titlecase(&therapy_class),
                        url: format!("/therapies/{}/index.html", therapy_class),
                    },
                ],
            },
            html! {
		div .full-width {
		    (markup_therapy(therapy, &mut therapy_markdown))
		}
	    },
        ))
    } else {
        Err(ErrorNotFound(format!(
            "Could not find page for therapy {}",
            therapy_name
        )))
    }
}

fn read_therapy(therapy_class: &String, therapy_name: &String) -> Result<Therapy, Box<dyn Error>> {
    let file = File::open(format!(
        "../articles/therapies/{}/{}/therapy.json",
        therapy_class, therapy_name
    ))?;

    let therapy = from_reader(file)?;
    Ok(therapy)
}

fn read_therapy_markdown(
    therapy_class: &String,
    therapy_name: &String,
) -> Result<ReferencedMarkup, IoError> {
    let path = format!(
        "../articles/therapies/{}/{}/therapy.md",
        therapy_class, therapy_name
    );

    file_to_markup(&path)
}

#[rustfmt::skip::macros(html)]
fn markup_therapy(therapy: Therapy, markdown: &mut ReferencedMarkup) -> Markup {
    html! {
	div .narrow-content {
	    section #overview {
		h1 {
		    (therapy.name)
		}
		@if !therapy.marketing_names.is_empty() {
		    p .subtitle {
			(format!("Also known as {}, marketed as {}", therapy.short_name, therapy.marketing_names.join(", ")))
		    }
		}
		@for paragraph in therapy.detailed {
		    p {
			(paragraph)
		    }
		}
	    }
	    section #indication {
		h2 .nospace {
		    (format!("What prostate cancer patients are eligible for {}?",
			     therapy.name))
		}
		p .subtitle {
		    strong {
			"NOTE: this encyclopedia currently covers only prostate cancer." br;
			"This treatment may or may not be indicated for other conditions."
		    }
		}
		@for paragraph in therapy.approvals_summary {
		    (direct_to_markup(&paragraph, &mut markdown.references))
		}
		@if !therapy.approvals.is_empty() {
		    ul {
			@for approval in therapy.approvals {
			    li {
				a href=(approval.reference) {
				    b {
					@if let Some(biomarker) = approval.biomarker {
					    (format!("{} with {}:",
						     approval.condition,
						     biomarker))
					} @else {
					    (format!("{}:", approval.condition))
					}
				    }
				}
				" This approval was based on the results of the "
				    a href=(format!("https://clinicaltrials.gov/study/{}",
						    approval.trial_nct_id)) {
					(format!("{} clinical trial", approval.trial_name))
				    }
				". "
				(direct_to_markup(&approval.description, &mut markdown.references))
			    }
			}
		    }
		}
	    }
	    section #evidence {
		(markdown.markup)
	    }
	    (generate_reference_list(&markdown.references))
	}
    }
}
