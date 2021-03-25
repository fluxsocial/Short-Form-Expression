use hc_time_index::IndexableEntry;
use hdk::prelude::*;
use chrono::{Utc, DateTime, NaiveDateTime};

use crate::ShortFormExpression;

impl IndexableEntry for ShortFormExpression {
    fn entry_time(&self) -> DateTime<Utc> {
        let now = sys_time().expect("Could not get sys time");
        let now = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(now.as_secs_f64() as i64, now.subsec_nanos()),
            Utc,
        );
        debug!("Now for entry: {:#?}", now);
        now
    }
    
    fn hash(&self) -> ExternResult<EntryHash> {
        hash_entry(self)
    }
}