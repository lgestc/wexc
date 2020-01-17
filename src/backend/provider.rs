use crate::model::entry::Entry;

pub trait Provider {
    fn provide_entries(&self) -> Vec<Entry>;

    fn report_for_entries(&self, entries: & Vec<&Entry>) -> String;
}
