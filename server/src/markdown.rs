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

use comrak::arena_tree::Node;
use comrak::nodes::{Ast, AstNode, NodeValue};
use comrak::{format_html, parse_document, Arena, Options};
use maud::{Markup, PreEscaped};

use crate::models::ReferencedMarkup;
use crate::reference::{get_reference, to_descriptor};

use std::cell::RefCell;
use std::fs::File;
use std::io::{Error, Read};

pub fn file_to_markup(path: &str) -> Result<ReferencedMarkup, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(markdown_to_markup(&contents))
}

fn iter_nodes<'a>(
    node: &'a AstNode<'a>,
    citations: &mut Vec<String>,
    arena: &'a Arena<AstNode<'a>>,
) {
    parse_citations(node, citations, arena);
    for c in node.children() {
        iter_nodes(c, citations, arena);
    }
}

fn parse_citations<'a>(
    node: &'a AstNode<'a>,
    citations: &mut Vec<String>,
    arena: &'a Arena<AstNode<'a>>,
) {
    let mut data = node.data.borrow_mut();
    match &mut data.value {
        &mut NodeValue::Link(ref mut link) => {
            if let Some(citation) = link.url.strip_prefix("#cite:") {
                citations.push(citation.to_string());

                let reference = get_reference(&citation.to_string());
                let reference_name = if let Ok(reference) = reference {
                    to_descriptor(reference)
                } else {
                    String::from("Missing citation")
                };

                if node.children().count() == 0 {
                    let ref_node = arena.alloc(Node::new(RefCell::new(Ast::new(
                        NodeValue::Text(reference_name),
                        data.sourcepos.start,
                    ))));

                    node.prepend(ref_node)
                }
            }
        }
        _ => (),
    }
}

#[rustfmt::skip::macros(html)]
pub fn markdown_to_markup(markdown: &str) -> ReferencedMarkup {
    let arena = Arena::new();

    let mut options = Options::default();
    options.extension.table = true;

    let root = parse_document(&arena, markdown, &options);

    let mut citations = Vec::new();
    iter_nodes(root, &mut citations, &arena);

    println!("citations: {:?}", citations);

    let mut html = vec![];
    format_html(root, &Options::default(), &mut html).unwrap();

    ReferencedMarkup {
        markup: PreEscaped(String::from_utf8(html).unwrap()),
        references: citations,
    }
}

pub fn direct_to_markup(markdown: &str, reference: &mut Vec<String>) -> Markup {
    let mut referenced = markdown_to_markup(markdown);

    reference.append(&mut referenced.references);
    referenced.markup
}
