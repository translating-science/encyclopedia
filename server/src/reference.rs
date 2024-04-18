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

use serde_json::de::from_reader;

use std::fs::File;
use std::io::Error;

use crate::models::Reference;

use maud::{html, Markup};

pub fn get_reference(ref_id: &String) -> Result<Reference, Error> {
    let file = File::open(format!("../articles/references/{}.json", ref_id))?;

    let reference = from_reader(file);

    if let Err(ref e) = reference {
        println!("Parsing reference {} failed: {:?}", ref_id, e);
    }
    Ok(reference?)
}

pub fn to_descriptor(reference: Reference) -> String {
    let authors = reference.authors.len();

    if authors == 1 {
        format!("{}, {}", reference.authors[0], reference.year)
    } else if authors == 2 {
        let author_str = format!("{} and {}", reference.authors[0], reference.authors[1]);
        format!("{}, {}", author_str, reference.year)
    } else if authors > 2 {
        format!("{} et al, {}", reference.authors[0], reference.year)
    } else {
        format!("{}, {}", reference.title, reference.year)
    }
}

#[rustfmt::skip::macros(html)]
pub fn generate_reference_list(references: &Vec<String>) -> Markup {
    html! {
	section #references {
	    h2 {
		"References"
	    }
	    ol {
		@for reference_id in references {
		    @if let Ok(reference) = get_reference(&reference_id) {
			(to_markup(&reference_id, reference))
		    }
		}
	    }
	}
    }
}

#[rustfmt::skip::macros(html)]
fn links(reference: Reference) -> Markup {
    html! {
	br;
	@if let Some(pmcid) = reference.pmcid {
	    a href=(format!("https://www.ncbi.nlm.nih.gov/pmc/articles/{}/", pmcid)) {
		"See article at PubMed Central"
	    }
	    " "
	} @else if let Some(pmid) = reference.pmid {
	    a href=(format!("https://www.ncbi.nlm.nih.gov/pubmed/articles/{}/", pmid)) {
		"See abstract at PubMed Central"
	    }
	    " "
	}
	a href=(reference.url) {
	    "See article at publisher's site"
	}
    }
}

#[rustfmt::skip::macros(html)]
fn to_markup(id: &String, reference: Reference) -> Markup {
    html! {
	li #(format!("cite:{}", id)) {
	    (format!("{}. ({}) ", reference.authors.join(", "),
		     reference.year))
	    @if let Some(ref journal) = reference.journal {
		(format!("'{}', ", reference.title))
		i {
		    (journal)
		}
		"."
		(links(reference))
	    } @else if let Some(ref conference) = reference.conference {
		(format!("'{}', Proceedings of ", reference.title))
		i {
		    (conference)
		}
		"."
		(links(reference))
	    } @else {
		a href=(reference.url) {
		    i {
			(reference.title)
		    }
		}
		"."
	    }
	}
    }
}
