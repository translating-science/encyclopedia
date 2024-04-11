# Encyclopedia of Precision Medicine

The rise of next-generation sequencing in clinical practice means that genomics
is no longer solely a research discipline: patients, caregivers, and clinicians
will routinely interact with genomic information.

A tremendous wave of open source bioinformatics software has sprung up over the
last decade, buoyed by a large investment in generating (often open) genomic
data. However, these tools and information are often only accessible to trained
researchers. The complex science and the vast amounts of scientific jargon
that surround both genomics and the medical disciplines that routinely use
genomic sequencing for diagnostic purposes, make it hard for patients to drive
their care decisions.

This repository contains the source code (both for documents and the server)
for the Encyclopedia of Precision Medicine. If you would like to browse the
encyclopedia, you can access a [hosted site at Translating Science
PBC](https://encyclopedia.translating.science).

Currently, we are focused on metastatic prostate cancer. We intend to expand
the encyclopedia over time, with the goal of covering all of oncology and then
expanding into other conditions.

## Design intent

The Encyclopedia of Precision Medicine has the goal of making this information
open and accessible to all people. Because of this, we [choose to architect the
encyclopedia](https://abbycovert.com/ia-tools/choose-your-words/) so that users
will describe the encyclopedia as:

1. Accessible
2. Trustworthy
3. Patient/care-centric

This means that users will probably not describe the encyclopedia as:

1. Academic
2. Biased towards newer technologies / treatments
3. Research-centric

In the spirit of the goal of making information accessible to all, our service
is designed for [accessibility](https://www.a11yproject.com). To this end, we:

* Use lightweight page designs that render rapidly and correctly on a wide range
  of devices and browsers.
* Follow accessible web best practices and validate our site against the
  [WCAG-2AAA accessibility
  guidelines](https://www.w3.org/WAI/standards-guidelines/wcag/).

## How to build locally

The web server for the Encyclopedia is easy to build and run locally. It is
a [Rust](https://www.rust-lang.org) service that depends on a minimal number of
Crates (libraries). Begin by installing [Rust and
Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Once you have Rust and Cargo installed, you can build and launch a server by
running the following commands:

```
cd server
cargo run
```

This should launch a server running on port 8081.
