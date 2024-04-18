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

use std::error::Error;
use std::fs::File;

use crate::models::Gene;
use crate::template::{render_page, NamedUrl, Navigation};

#[rustfmt::skip::macros(html)]
#[get("/genes/{gene}/index.html")]
pub async fn gene_page(path: web::Path<String>) -> Result<Markup> {
    let gene_symbol = path.into_inner();
    let gene_result = read_gene_json(&gene_symbol);

    if let Ok(gene) = gene_result {
        let gene_title = format!("{} gene", gene_symbol);

        Ok(render_page(
            &gene_title,
            &Navigation {
                page_title: gene_title.clone(),
                parents: vec![
                    NamedUrl {
                        name: String::from("Encyclopedia of Precision Medicine"),
                        url: String::from("/"),
                    },
                    NamedUrl {
                        name: String::from("Genes"),
                        url: String::from("/genes/index.html"),
                    },
                ],
            },
            html! {
		div .full-width {
		    (markup_gene(gene))
		}
	    },
        ))
    } else {
        Err(ErrorNotFound(format!(
            "Could not find page for gene {}",
            gene_symbol
        )))
    }
}

fn read_gene_json(gene_name: &String) -> Result<Gene, Box<dyn Error>> {
    let file = File::open(format!("../articles/genes/{}/gene.json", gene_name))?;

    let gene = from_reader(file)?;
    Ok(gene)
}

#[rustfmt::skip::macros(html)]
fn markup_gene(gene: Gene) -> Markup {
    html! {
	div .narrow-content {
	    section #overview {
		h1 {
		    (format!("{} gene", gene.symbol))
		}
		h2 {
		    (gene.full_name)
		}
		@for paragraph in gene.detailed {
		    p {
			(paragraph)
		    }
		}
	    }
	}
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_read_gene_json() {
        use crate::gene::read_gene_json;

        let gene_result = read_gene_json(&String::from("NF1"));
        assert!(gene_result.is_ok());

        if let Ok(gene) = gene_result {
            assert_eq!(gene.symbol, String::from("NF1"));
            assert_eq!(gene.detailed.len(), 2);
            assert_eq!(gene.location, String::from("chromosome 17"));
            assert_eq!(gene.alternative_names.len(), 2);
            assert_eq!(gene.pathways.len(), 6);
        }
    }

    #[test]
    fn test_read_missing_gene_json() {
        use crate::gene::read_gene_json;

        // there is no BRCA gene, there exist BRCA1 and BRCA2
        let gene_result = read_gene_json(&String::from("BRCA"));
        assert!(gene_result.is_err());
    }

    #[test]
    fn test_gene_markup() {
        use crate::gene::markup_gene;
        use crate::models::Gene;

        let gene = Gene {
            symbol: String::from("MyGene"),
            full_name: String::from("My favorite gene"),
            detailed: vec![String::from("MyGene is an example gene.")],
            location: String::from("Chromosome Z"),
            alternative_names: Vec::new(),
            pathways: Vec::new(),
        };

        let expected_markup = r#"<div class="narrow-content"><section id="overview"><h1>MyGene gene</h1><h2>My favorite gene</h2><p>MyGene is an example gene.</p></section></div>"#;
        let gene_markup = markup_gene(gene);

        assert_eq!(gene_markup.into_string(), expected_markup);
    }
}
