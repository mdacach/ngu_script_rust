use std::path::Path;

pub fn get_ocr_text(path_as_str: &str) -> Option<String> {
    let mut ocr = leptess::tesseract::TessApi::new(None, "eng").unwrap();

    let pix = leptess::leptonica::pix_read(Path::new(path_as_str)).unwrap();
    ocr.set_image(&pix);

    if let Ok(text) = ocr.get_utf8_text() {
        Some(text)
    } else {
        None
    }
}

#[test]
fn test() {}
