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

use maud::{html, Markup};
use serde_json::de::from_reader;

use std::error::Error;
use std::fs::File;

use crate::markdown::file_to_markup;
use crate::models::{Article, ArticleMarkup};
use crate::reference::generate_reference_list;

pub fn read_article(
    category_name: &String,
    article_name: &String,
) -> Result<Article, Box<dyn Error>> {
    let file = File::open(format!(
        "../articles/{}/{}/article.json",
        category_name, article_name
    ))?;

    let article = from_reader(file)?;
    Ok(article)
}

pub fn read_article_markup(
    category_name: &String,
    article_name: &String,
) -> Result<ArticleMarkup, Box<dyn Error>> {
    let article = read_article(category_name, article_name)?;

    match file_to_markup(&format!(
        "../articles/{}/{}/article.md",
        category_name, article_name
    )) {
        Ok(markup) => Ok(ArticleMarkup {
            article: article,
            markup: Some(markup),
        }),
        Err(_) => Ok(ArticleMarkup {
            article: article,
            markup: None,
        }),
    }
}

#[rustfmt::skip::macros(html)]
pub fn markup_article(article: ArticleMarkup) -> Markup {
    html! {
	div .narrow-content {
	    h1 {
		(article.article.name)
	    }
	    @if let Some(alternative_names) = article.article.alternative_names {
		p .subtitle {
		    (format!("Also known as {}", alternative_names.join(", ")))
		}
	    }
	    @if let Some(markup) = article.markup {
		(markup.markup)
		@if !markup.references.is_empty() {
		    (generate_reference_list(&markup.references))
		}
	    }
	    @else if let Some(stub_issue) = article.article.stub_issue {
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
