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

use crate::template::{render_home, NamedUrl};

#[rustfmt::skip::macros(html)]
#[get("/index.html")]
pub async fn landing_page() -> Result<Markup> {
    Ok(render_home(html! {
	    div .narrow-content {
		section #epm-overview {
		    p {
			"The ambition of Precision Medicine is to help patients live "
			"longer and healthier lives by matching them to the "
			"best treatments for their personal condition, biology, "
			"and personal history."
		    }
		    p {
			"The Encyclopedia of Precision Medicine makes knowledge about "
			"precision medicine freely accessible to everyone by "
			"summarizing the best scientific knowledge about genomics, "
			"conditions, treatments."
		    }
		}
	    }
	    (browse(get_sections()))
	}))
}

fn get_sections() -> Vec<NamedUrl> {
    vec![NamedUrl {
        name: String::from("Genes"),
        url: String::from("/genes/index.html"),
    }]
}

#[rustfmt::skip::macros(html)]
fn browse(sections: Vec<NamedUrl>) -> Markup {
    if sections.is_empty() {
        panic!("Can't create section nav without sections.")
    }

    html! {
	div .narrow-content {
	    div .light-background-color {
		nav #epm-sections {
		    h2 .dark-background-color .light-font-color .bright-top-border {
			"Browse sections of the Encyclopedia of Precision Medicine"
		    }
		    div {
			ol {
			    @for section in sections {
				li {
				    a href=(section.url) {
					(section.name)
				    }
				}
			    }
			}
		    }
		}
	    }
	}
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_sections() {
        use crate::landing::get_sections;
        assert_eq!(get_sections().len(), 1)
    }

    #[test]
    #[should_panic(expected = "Can't create section nav without sections.")]
    fn test_browse_empty() {
        use crate::landing::browse;

        browse(Vec::new());
    }
}
