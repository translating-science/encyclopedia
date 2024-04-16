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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pathway {
    pub pathway: String,
    pub references: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Gene {
    pub symbol: String,
    pub full_name: String,
    pub detailed: Vec<String>,
    pub location: String,
    pub alternative_names: Vec<String>,
    pub pathways: Vec<Pathway>,
}

#[derive(Serialize, Deserialize)]
pub struct Approval {
    pub reference: String,
    pub trial_name: String,
    pub trial_nct_id: String,
    pub condition: String,
    pub biomarker: Option<String>,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Evidence {
    pub trial_name: Option<String>,
    pub trial_nct_id: Option<String>,
    pub assessed: String,
    pub rationale: String,
    pub result: String,
    pub references: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Therapy {
    pub name: String,
    pub marketing_names: Vec<String>,
    pub rxcui: String,
    pub detailed: Vec<String>,
    pub approvals_summary: Vec<String>,
    pub approvals: Vec<Approval>,
    pub evidence_summary: Vec<String>,
    pub evidence: Vec<Evidence>,
}
