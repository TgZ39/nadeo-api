// Contains metadata needed to execute a `NadeoRequest`.
// Only contains a UserAgent right now.
#[derive(Clone, Debug)]
pub(crate) struct MetaData {
    pub(crate) user_agent: String,
}
