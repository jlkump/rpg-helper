pub trait GalleryData {
    fn get_display_name(&self) -> &str;
    fn get_display_img_src(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_description_brief(&self) -> &str;
    fn get_tags(&self) -> Vec<String>;
    fn get_version(&self) -> Option<&str>;
    fn get_id(&self) -> uuid::Uuid;
}