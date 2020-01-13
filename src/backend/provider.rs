use crate::model::entry::Entry;

pub trait Provider {
    fn provide_entries(&self) -> Vec<Entry>;
}
