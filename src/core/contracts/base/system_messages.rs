
pub trait SysMessage {
    fn get_internal_message(&self) -> &str;
}

#[derive(Clone)]
pub struct InformationMessage {
    pub(crate) message: String
}

impl SysMessage for InformationMessage {
    fn get_internal_message(&self) -> &str {
        return &self.message;
    }
}
