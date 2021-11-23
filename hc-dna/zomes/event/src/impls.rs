use hc_time_index::IndexableEntry;
use hdk::prelude::*;
use chrono::{Utc, DateTime};

use crate::EventExpression;

impl IndexableEntry for EventExpression {
    fn entry_time(&self) -> DateTime<Utc> {
        self.timestamp
    }
    
    fn hash(&self) -> ExternResult<EntryHash> {
        hash_entry(self)
    }
}