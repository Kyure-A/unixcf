use base64::{engine::general_purpose, Engine};
use image::{ImageBuffer, Rgba};
use xcf::{PropertyIdentifier, PropertyPayload, Xcf};

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
pub extern "C" fn bakeImage(path: String) -> String {
    let xcf = Xcf::open(path).unwrap();
    let mut image = ImageBuffer::new(xcf.width(), xcf.height());
    
    for layer in xcf.layers.iter().rev() {
        let prop_offsets = layer.properties
            .iter()
            .find(|l| l.kind == PropertyIdentifier::PropOffsets)
            .unwrap()
            .get_prop_offsets();

        let rgba_buf = &layer
            .pixels
            .pixels;

        for pixel in rgba_buf.iter().enumerate() {
            let pixel_height = pixel.0 as i32 / layer.width as i32;
            let pixel_width = pixel.0 as i32 - pixel_height * layer.width as i32;
            
            // pixel.0 は単純に index だから、1980, 21 の座標の pixel は 1980 + 100 - 1 で 2000 である
            // そのため x 座標を出すには index - height をすべし

            let offsetted_width = (pixel_width + prop_offsets.width) as u32;
            let offsetted_height = (pixel_height + prop_offsets.height) as u32;
            
            // 画像からはみでた layer 部分には書き込まない
            // layer の透過部分はかきこまない
            if pixel.1.a() == 0
                || offsetted_width <= 0 || xcf.width() <= offsetted_width 
                || offsetted_height <= 0 || xcf.height() <= offsetted_height {
                    continue;
                }
            
            let rgba = Rgba([pixel.1.r(), pixel.1.g(), pixel.1.b(), pixel.1.a()]);
            
            image.put_pixel(offsetted_width, offsetted_height, rgba);
        }
    }

    let b64 = general_purpose::STANDARD.encode(image.into_vec());

    return b64;
}
