mod message;

pub mod did;
pub mod error;
pub mod pack;
pub mod secrets;

pub use message::{
    Attachment, AttachmentBuilder, AttachmentData, Base64AttachmentData, JsonAttachmentData,
    LinksAttachmentData, Message, MessageBuilder,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
