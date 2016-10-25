// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod create {
    use std::io;
    use std::fs::File;
    use std::path::Path;

    use common::ui::{Status, UI};

    use error::Result;

    pub fn start(ui: &mut UI,
                 pkg_name: &str) -> Result<()> {
        try!(ui.begin(format!("Lets make a plan for {}",
                              &pkg_name)));
        Ok(())
    }
}
