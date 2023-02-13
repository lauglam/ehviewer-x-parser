#[derive(Debug, PartialEq)]
pub struct GalleryPreviewMedium {
    pub position: u32,
    pub filename: String,
    pub page_url: String,
    pub image_url: String,
    /// in the medium, the preview is a mosaic of 20 images.
    pub offset_x: u32,
    pub offset_y: u32,
    pub clip_width: u32,
    pub clip_height: u32,
}
