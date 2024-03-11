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

use maud::{html, Markup, PreEscaped, DOCTYPE};

use std::env;

/// A struct mapping a short text name for a link to a URL.
pub struct NamedUrl {
    /// Short text name describing this URL.
    pub name: String,
    /// The URL to link to.
    pub url: String,
}

/// A struct describing navigation across pages.
pub struct Navigation {
    /// The name of this page.
    pub page_title: String,
    /// The pages logically higher in the hierarchy than this page.
    pub parents: Vec<NamedUrl>,
}

/// Adds breadcrumb navigation to a page.
///
/// # Arguments
/// * `navigation` A struct containing navigation info.
#[rustfmt::skip::macros(html)]
fn nav(navigation: &Navigation) -> Markup {
    html! {
	div .full-width .white-background-color {
	    div .narrow-content {
		nav aria-label="Breadcrumb" class="breadcrumb" {
		    ol {
			@for parent in &navigation.parents {
			    li .crumb {
				a href=(parent.url) {
				    (parent.name)
				}
			    }
			}
			li .crumb {
			    span aria-current="page" {
				(navigation.page_title)
			    }
			}
		    }
		}
	    }
	}
    }
}

#[rustfmt::skip::macros(html)]
fn posthog(posthog_project: String, posthog_url: String) -> Markup {
    html! {
	script {
	    (PreEscaped(r#"!function(t,e){var o,n,p,r;e.__SV||(window.posthog=e,e._i=[],e.init=function(i,s,a){function g(t,e){var o=e.split(".");2==o.length&&(t=t[o[0]],e=o[1]),t[e]=function(){t.push([e].concat(Array.prototype.slice.call(arguments,0)))}}(p=t.createElement("script")).type="text/javascript",p.async=!0,p.src=s.api_host+"/static/array.js",(r=t.getElementsByTagName("script")[0]).parentNode.insertBefore(p,r);var u=e;for(void 0!==a?u=e[a]=[]:a="posthog",u.people=u.people||[],u.toString=function(t){var e="posthog";return"posthog"!==a&&(e+="."+a),t||(e+=" (stub)"),e},u.people.toString=function(){return u.toString(1)+".people (stub)"},o="capture identify alias people.set people.set_once set_config register register_once unregister opt_out_capturing has_opted_out_capturing opt_in_capturing reset isFeatureEnabled onFeatureFlags getFeatureFlag getFeatureFlagPayload reloadFeatureFlags group updateEarlyAccessFeatureEnrollment getEarlyAccessFeatures getActiveMatchingSurveys getSurveys onSessionId".split(" "),n=0;n<o.length;n++)g(u,o[n]);e._i.push([i,s,a])},e.__SV=1)}(document,window.posthog||[]);"#))
	    (PreEscaped(format!("posthog.init('{}',{{api_host:'{}'}})",
				posthog_project,
				posthog_url)))
	}
    }
}

/// Generates the head metadata section for a page.
///
/// # Arguments
/// * `title` Name of the page to go in the header.
#[rustfmt::skip::macros(html)]
pub fn head(title: Option<String>) -> Markup {
    html! {
	(DOCTYPE)
            head {
		meta charset="utf-8";
		meta name="viewport" content="width=device-width,initial-scale=1";
		@if let Some(title) = title {
		    title { (format!("Encyclopedia of Precision Medicine: {}—Translating Science PBC", title)) }
		} @else {
		    title { "Encyclopedia of Precision Medicine—Translating Science PBC" }
		}
		link rel="stylesheet" href="/resources/styles.css";
		link rel="apple-touch-icon" sizes="180x180" href="/resources/apple-touch-icon.png";
		link rel="icon" type="image/png" sizes="32x32" href="/resources/favicon-32x32.png";
		link rel="icon" type="image/png" sizes="16x16" href="/resources/favicon-16x16.png";
		link rel="manifest" href="/resources/site.webmanifest";
		link rel="mask-icon" href="/resources/safari-pinned-tab.svg" color="#6C8DFF";
		meta name="apple-mobile-web-app-title" content="Encyclopedia of Precision Medicine—Translating Science PBC";
		meta name="application-name" content="Encyclopedia of Precision Medicine—Translating Science PBC";
		meta name="msapplication-TileColor" content="#6C8DFF";
		meta name="theme-color" content="#6C8DFF";
		script src="https://unpkg.com/htmx.org@1.9.10" {};
		@if let Some(posthog_project) = env::var_os("POSTHOG_PROJECT") {
		    @if let Some(posthog_url) = env::var_os("POSTHOG_URL") {
			@if let Ok(posthog_project) = posthog_project.into_string() {
			    @if let Ok(posthog_url) = posthog_url.into_string() {
				(posthog(posthog_project, posthog_url))
			    }
			}
		    }
		}
            }
    }
}

/// Generates the header displayed visually at the top of the page.
///
/// # Arguments
/// * `is_h1` If true, marks the header text as a heading. Should only
///   be used for landing page.
#[rustfmt::skip::macros(html)]
pub fn header(is_h1: bool) -> Markup {
    let tagline = html! {
	"Encyclopedia of "
	br #logo-br;
	"Precision Medicine"
    };

    html! {
	header {
            div .dark-background-color #header {
		div {
		    a href="#container" id="skip-link" tabindex="0" {
			"Skip to main content"
		    }
		}
		div #logo {
		    a href="https://translating.science" .header-footer-link {
			div #logo-initials {
			    "TS"
			}
		    }
		    a href="/" .header-footer-link {
			div #logo-small {
			    @if is_h1 {
				h1 {
				    (tagline)
				}
			    } @else {
				(tagline)
			    }
			}
		    }
		}
            }
	}
    }
}

/// Generates the footer displayed visually at the bottom of the page.
///
/// # Arguments
/// - `include_cc` If true, includes the Creative Commons license footer.
///   This should be true for all pages except for the home/landing page.
#[rustfmt::skip::macros(html)]
pub fn footer(include_cc: bool) -> Markup {
    html! {
	footer {
	    @if include_cc {
		div .white-background-color #license {
		    div .narrow-content {
			p {
			    "The "
				a href="/" {
				    "Precision Medicine Encyclopedia"
				}
			    " by "
				a href="https://translating.science" {
				    "Translating Science PBC"
				}
			    " is licensed under "
				a href="http://creativecommons.org/licenses/by-sa/4.0/" target="_blank" rel="license noopener noreferrer" {
				    "CC BY-SA 4.0"
					img src="/resources/cc.svg" {}
				    img src="/resources/by.svg" {}
				    img src="/resources/sa.svg" {}
				}
			}
		    }
		}
	    }
            div .dark-background-color .light-font-color {
		div .narrow-content {
		    div .footer-text {
			p {
			    span .footer-brand {
				"Translating Science PBC"
			    } br;
			    "© 2024. All rights reserved." br;
			    a href="https://translating.science/terms.html" .light-link {
				"Terms of Service"
			    } br;
			    a href="https://translating.science/privacy-policy.html" .light-link {
				"Privacy Policy"
			    } br;
			    a href="mailto:info@translating.science" .light-link {
				"Contact us"
			    }
			}
			div {
			    a href="https://www.linkedin.com/company/translating-science-pbc/" {
				img src="/resources/LI-In-Bug-white.png" .footer-icons
				    alt="Translating Science PBC on LinkedIn" {}
			    }
			    a href="https://github.com/translating-science" {
				img src="/resources/github-mark-white.png" .footer-icons
				    alt="Translating Science PBC on GitHub" {}
			    }
			}
		    }
		}
	    }
	    div #footer-fade {
	    }
	}
    }
}

/// Renders the whole page.
///
/// Wraps the provided markdown in page metadata, header/footer, and main section.
/// Adds a navigation breadcrumb for users.
///
/// # Arguments
/// * `title` The page title to use in the HTML head title block.
/// * `navigation` Navigation struct used to populate nav breadcrumbs.
/// * `page_body` The markup to include as the main page content.
#[rustfmt::skip::macros(html)]
pub fn render_page(title: &String, navigation: &Navigation, page_body: Markup) -> Markup {
    html! {
	html lang="en" {
	(head(Some(title.clone())))
        body {
            (header(false))
		(nav(navigation))
		main #container tabindex="-1" {
			(page_body)
		}
            (footer(true))
	}
	}
    }
}

/// Renders the home page.
///
/// Similar to `render_page` but omits breadcrumb navigation and uses a
/// stripped down header.
///
/// # Arguments
/// * `page_body` The markup to include as the main page content.
#[rustfmt::skip::macros(html)]
pub fn render_home(page_body: Markup) -> Markup {
    html! {
	html lang="en" {
	(head(None))
        body {
            (header(true))
		main #container tabindex="-1" {
		    (page_body)
		}
            (footer(false))
	}
	}
    }
}

/// Generates an indented div explaining a query fetch error.
///
/// # Arguments
/// * `description` A brief description of what we failed to fetch.
#[rustfmt::skip::macros(html)]
pub fn error_html(description: String) -> Markup {
    html! {
	div .indented-item .large-text {
            (format!("There was an error fetching {}.", description))
	}
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_error_html() {
        use crate::template::error_html;

        let error_markup = error_html(String::from("home"));

        let expected_markup =
            r#"<div class="indented-item large-text">There was an error fetching home.</div>"#;

        assert_eq!(error_markup.into_string(), expected_markup);
    }

    #[test]
    fn test_head_with_title() {
        use crate::template::head;

        let head_markup = head(Some(String::from("Home")));

        let expected_markup = r##"<!DOCTYPE html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Encyclopedia of Precision Medicine: Home—Translating Science PBC</title><link rel="stylesheet" href="/resources/styles.css"><link rel="apple-touch-icon" sizes="180x180" href="/resources/apple-touch-icon.png"><link rel="icon" type="image/png" sizes="32x32" href="/resources/favicon-32x32.png"><link rel="icon" type="image/png" sizes="16x16" href="/resources/favicon-16x16.png"><link rel="manifest" href="/resources/site.webmanifest"><link rel="mask-icon" href="/resources/safari-pinned-tab.svg" color="#6C8DFF"><meta name="apple-mobile-web-app-title" content="Encyclopedia of Precision Medicine—Translating Science PBC"><meta name="application-name" content="Encyclopedia of Precision Medicine—Translating Science PBC"><meta name="msapplication-TileColor" content="#6C8DFF"><meta name="theme-color" content="#6C8DFF"><script src="https://unpkg.com/htmx.org@1.9.10"></script></head>"##;

        assert_eq!(head_markup.into_string(), expected_markup);
    }

    #[test]
    fn test_head_no_title() {
        use crate::template::head;

        let head_markup = head(None);

        let expected_markup = r##"<!DOCTYPE html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Encyclopedia of Precision Medicine—Translating Science PBC</title><link rel="stylesheet" href="/resources/styles.css"><link rel="apple-touch-icon" sizes="180x180" href="/resources/apple-touch-icon.png"><link rel="icon" type="image/png" sizes="32x32" href="/resources/favicon-32x32.png"><link rel="icon" type="image/png" sizes="16x16" href="/resources/favicon-16x16.png"><link rel="manifest" href="/resources/site.webmanifest"><link rel="mask-icon" href="/resources/safari-pinned-tab.svg" color="#6C8DFF"><meta name="apple-mobile-web-app-title" content="Encyclopedia of Precision Medicine—Translating Science PBC"><meta name="application-name" content="Encyclopedia of Precision Medicine—Translating Science PBC"><meta name="msapplication-TileColor" content="#6C8DFF"><meta name="theme-color" content="#6C8DFF"><script src="https://unpkg.com/htmx.org@1.9.10"></script></head>"##;

        assert_eq!(head_markup.into_string(), expected_markup);
    }

    #[test]
    fn test_header_not_heading() {
        use crate::template::header;

        let header_markup = header(false);

        let expected_markup = r##"<header><div class="dark-background-color" id="header"><div><a href="#container" id="skip-link" tabindex="0">Skip to main content</a></div><div id="logo"><a class="header-footer-link" href="https://translating.science"><div id="logo-initials">TS</div></a><a class="header-footer-link" href="/"><div id="logo-small">Encyclopedia of <br id="logo-br">Precision Medicine</div></a></div></div></header>"##;

        assert_eq!(header_markup.into_string(), expected_markup);
    }

    #[test]
    fn test_header_is_heading() {
        use crate::template::header;

        let header_markup = header(true);

        let expected_markup = r##"<header><div class="dark-background-color" id="header"><div><a href="#container" id="skip-link" tabindex="0">Skip to main content</a></div><div id="logo"><a class="header-footer-link" href="https://translating.science"><div id="logo-initials">TS</div></a><a class="header-footer-link" href="/"><div id="logo-small"><h1>Encyclopedia of <br id="logo-br">Precision Medicine</h1></div></a></div></div></header>"##;

        assert_eq!(header_markup.into_string(), expected_markup);
    }

    #[test]
    fn test_footer_cc() {
        use crate::template::footer;

        let footer_markup = footer(true);

        let expected_markup = r#"<footer><div class="white-background-color" id="license"><div class="narrow-content"><p>The <a href="/">Precision Medicine Encyclopedia</a> by <a href="https://translating.science">Translating Science PBC</a> is licensed under <a href="http://creativecommons.org/licenses/by-sa/4.0/" target="_blank" rel="license noopener noreferrer">CC BY-SA 4.0<img src="/resources/cc.svg"></img><img src="/resources/by.svg"></img><img src="/resources/sa.svg"></img></a></p></div></div><div class="dark-background-color light-font-color"><div class="narrow-content"><div class="footer-text"><p><span class="footer-brand">Translating Science PBC</span><br>© 2024. All rights reserved.<br><a class="light-link" href="https://translating.science/terms.html">Terms of Service</a><br><a class="light-link" href="https://translating.science/privacy-policy.html">Privacy Policy</a><br><a class="light-link" href="mailto:info@translating.science">Contact us</a></p><div><a href="https://www.linkedin.com/company/translating-science-pbc/"><img class="footer-icons" src="/resources/LI-In-Bug-white.png" alt="Translating Science PBC on LinkedIn"></img></a><a href="https://github.com/translating-science"><img class="footer-icons" src="/resources/github-mark-white.png" alt="Translating Science PBC on GitHub"></img></a></div></div></div></div><div id="footer-fade"></div></footer>"#;

        assert_eq!(footer_markup.into_string(), expected_markup);
    }

    #[test]
    fn test_footer_no_cc() {
        use crate::template::footer;

        let footer_markup = footer(false);

        let expected_markup = r#"<footer><div class="dark-background-color light-font-color"><div class="narrow-content"><div class="footer-text"><p><span class="footer-brand">Translating Science PBC</span><br>© 2024. All rights reserved.<br><a class="light-link" href="https://translating.science/terms.html">Terms of Service</a><br><a class="light-link" href="https://translating.science/privacy-policy.html">Privacy Policy</a><br><a class="light-link" href="mailto:info@translating.science">Contact us</a></p><div><a href="https://www.linkedin.com/company/translating-science-pbc/"><img class="footer-icons" src="/resources/LI-In-Bug-white.png" alt="Translating Science PBC on LinkedIn"></img></a><a href="https://github.com/translating-science"><img class="footer-icons" src="/resources/github-mark-white.png" alt="Translating Science PBC on GitHub"></img></a></div></div></div></div><div id="footer-fade"></div></footer>"#;

        assert_eq!(footer_markup.into_string(), expected_markup);
    }

    #[rustfmt::skip::macros(html)]
    #[test]
    fn test_render_page() {
        use crate::template::{render_page, Navigation};
        use maud::html;

        let nav = Navigation {
            page_title: String::from("A page"),
            parents: Vec::new(),
        };

        let page_markup = render_page(
            &String::from("A page"),
            &nav,
            html! {
                section #a-section {
                    "Hello"
                }
            },
        );

        let expected_markup = r##"<html lang="en"><!DOCTYPE html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Encyclopedia of Precision Medicine: A page—Translating Science PBC</title><link rel="stylesheet" href="/resources/styles.css"><link rel="apple-touch-icon" sizes="180x180" href="/resources/apple-touch-icon.png"><link rel="icon" type="image/png" sizes="32x32" href="/resources/favicon-32x32.png"><link rel="icon" type="image/png" sizes="16x16" href="/resources/favicon-16x16.png"><link rel="manifest" href="/resources/site.webmanifest"><link rel="mask-icon" href="/resources/safari-pinned-tab.svg" color="#6C8DFF"><meta name="apple-mobile-web-app-title" content="Encyclopedia of Precision Medicine—Translating Science PBC"><meta name="application-name" content="Encyclopedia of Precision Medicine—Translating Science PBC"><meta name="msapplication-TileColor" content="#6C8DFF"><meta name="theme-color" content="#6C8DFF"><script src="https://unpkg.com/htmx.org@1.9.10"></script></head><body><header><div class="dark-background-color" id="header"><div><a href="#container" id="skip-link" tabindex="0">Skip to main content</a></div><div id="logo"><a class="header-footer-link" href="https://translating.science"><div id="logo-initials">TS</div></a><a class="header-footer-link" href="/"><div id="logo-small">Encyclopedia of <br id="logo-br">Precision Medicine</div></a></div></div></header><div class="full-width white-background-color"><div class="narrow-content"><nav aria-label="Breadcrumb" class="breadcrumb"><ol><li class="crumb"><span aria-current="page">A page</span></li></ol></nav></div></div><main id="container" tabindex="-1"><section id="a-section">Hello</section></main><footer><div class="white-background-color" id="license"><div class="narrow-content"><p>The <a href="/">Precision Medicine Encyclopedia</a> by <a href="https://translating.science">Translating Science PBC</a> is licensed under <a href="http://creativecommons.org/licenses/by-sa/4.0/" target="_blank" rel="license noopener noreferrer">CC BY-SA 4.0<img src="/resources/cc.svg"></img><img src="/resources/by.svg"></img><img src="/resources/sa.svg"></img></a></p></div></div><div class="dark-background-color light-font-color"><div class="narrow-content"><div class="footer-text"><p><span class="footer-brand">Translating Science PBC</span><br>© 2024. All rights reserved.<br><a class="light-link" href="https://translating.science/terms.html">Terms of Service</a><br><a class="light-link" href="https://translating.science/privacy-policy.html">Privacy Policy</a><br><a class="light-link" href="mailto:info@translating.science">Contact us</a></p><div><a href="https://www.linkedin.com/company/translating-science-pbc/"><img class="footer-icons" src="/resources/LI-In-Bug-white.png" alt="Translating Science PBC on LinkedIn"></img></a><a href="https://github.com/translating-science"><img class="footer-icons" src="/resources/github-mark-white.png" alt="Translating Science PBC on GitHub"></img></a></div></div></div></div><div id="footer-fade"></div></footer></body></html>"##;

        assert_eq!(page_markup.into_string(), expected_markup);
    }

    #[test]
    fn test_navigation_no_children() {
        use crate::template::{nav, NamedUrl, Navigation};

        let no_children = Navigation {
            page_title: String::from("Tertiary page"),
            parents: vec![
                NamedUrl {
                    name: String::from("Homepage"),
                    url: String::from("/"),
                },
                NamedUrl {
                    name: String::from("Second level page"),
                    url: String::from("/second"),
                },
            ],
        };

        let nav_markup = nav(&no_children);
        let expected_markup = r##"<div class="full-width white-background-color"><div class="narrow-content"><nav aria-label="Breadcrumb" class="breadcrumb"><ol><li class="crumb"><a href="/">Homepage</a></li><li class="crumb"><a href="/second">Second level page</a></li><li class="crumb"><span aria-current="page">Tertiary page</span></li></ol></nav></div></div>"##;

        assert_eq!(nav_markup.into_string(), expected_markup);
    }
}
