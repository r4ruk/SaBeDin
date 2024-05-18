use log::info;

pub trait SysMessage {
    fn get_internal_message(&self) -> &str;
}

pub struct InformationMessage {
    pub(crate) message: String
}

impl SysMessage for InformationMessage {
    fn get_internal_message(&self) -> &str {
        return &self.message;
    }
}
