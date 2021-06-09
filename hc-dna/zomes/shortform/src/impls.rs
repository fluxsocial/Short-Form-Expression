use hc_time_index::IndexableEntry;
use hdk::prelude::*;
use chrono::{Utc, DateTime};

use crate::ShortFormExpression;

impl IndexableEntry for ShortFormExpression {
    fn entry_time(&self) -> DateTime<Utc> {
        self.timestamp
    }
    
    fn hash(&self) -> ExternResult<EntryHash> {
        hash_entry(self)
    }
}