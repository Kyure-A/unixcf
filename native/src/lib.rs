use base64::{engine::general_purpose, Engine};
use std::io::Cursor;
use image::{ImageBuffer, Rgba, ImageFormat, DynamicImage};
use xcf::{PropertyIdentifier, PropertyPayload, Xcf};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

struct XcfPropOffsets {
    width: i32,
    height: i32,
}

trait XcfProperty {
    fn get_prop_offsets(&self) -> XcfPropOffsets;
}

impl XcfProperty for xcf::Property {
    fn get_prop_offsets(&self) -> XcfPropOffsets {
        let payload = &self.payload;
        match payload {
            PropertyPayload::Unknown(data) => {
                if data.len() != 8 {
                    panic!("Property payload length is not appropriate");
                }
                
                let w = (data[0] as i32) << 24
                    | (data[1] as i32) << 16
                    | (data[2] as i32) << 8
                    | data[3] as i32;

                let h = (data[4] as i32) << 24
                    | (data[5] as i32) << 16
                    | (data[6] as i32) << 8
                    | data[7] as i32;
                
                return XcfPropOffsets {width: w, height: h};
            }
            _ => {
                panic!("Property payload is not appropriate");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn bakeImage(path: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let xcf = match Xcf::open(path_str) {
        Ok(x) => x,
        Err(_) => return std::ptr::null_mut(),
    };

    let mut image = ImageBuffer::new(xcf.width(), xcf.height());
    
    for layer in xcf.layers.iter().rev() {
        let prop_offsets = layer.properties
            .iter()
            .find(|l| l.kind == PropertyIdentifier::PropOffsets)
            .map(|prop| prop.get_prop_offsets())
            .unwrap_or(XcfPropOffsets { width: 0, height: 0 });

        let rgba_buf = &layer.pixels.pixels;

        for (idx, pixel) in rgba_buf.iter().enumerate() {
            let pixel_height = idx as i32 / layer.width as i32;
            let pixel_width = idx as i32 - pixel_height * layer.width as i32;

            let offsetted_width = (pixel_width + prop_offsets.width) as u32;
            let offsetted_height = (pixel_height + prop_offsets.height) as u32;

            if pixel.a() == 0
                || offsetted_width >= xcf.width()
                || offsetted_height >= xcf.height() {
                    continue;
                }

            let rgba = Rgba([pixel.r(), pixel.g(), pixel.b(), pixel.a()]);
            image.put_pixel(offsetted_width, offsetted_height, rgba);
        }
    }
    
    let mut cursor = Cursor::new(Vec::new());
    
    let dyn_img = DynamicImage::ImageRgba8(image);
    dyn_img.write_to(&mut cursor, ImageFormat::Png).unwrap();
    
    let buf = cursor.into_inner();
    
    let b64 = general_purpose::STANDARD.encode(buf);
    CString::new(b64).unwrap().into_raw()
}
