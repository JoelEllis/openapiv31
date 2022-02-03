use std::collections::HashMap;

use crate::{PathItem, ReferenceOr};

pub type Webhooks = HashMap<String, ReferenceOr<PathItem>>;
