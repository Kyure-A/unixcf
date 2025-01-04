use base64::{engine::general_purpose, Engine};
use image::{ImageBuffer, Rgba};
use xcf::{PropertyIdentifier, PropertyPayload, Xcf};

struct XcfPropOffsets {
    width: i32,
    height: i32,
}

trait XcfPropertyInt {
    fn get_prop_offsets(&self) -> XcfPropOffsets;
}

impl XcfPropertyInt for xcf::Property {
    fn get_prop_offsets(&self) -> XcfPropOffsets {
        let payload = &self.payload;
        match payload {
            PropertyPayload::Unknown(data) => {
                if data.len() != 8 {
                    panic!("Property payload length is not appropriate");
                }
                
                let x = (data[0] as i32) << 24
                    | (data[1] as i32) << 16
                    | (data[2] as i32) << 8
                    | data[3] as i32;

                let y = (data[4] as i32) << 24
                    | (data[5] as i32) << 16
                    | (data[6] as i32) << 8
                    | data[7] as i32;
                
                return XcfPropOffsets {width: x, height: y};
            }
            _ => {
                panic!("Property payload is not appropriate");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn bakeImage(path: String) -> String {
    let raw_image = Xcf::open(path).unwrap();
    let image_width = raw_image.width();
    let image_height = raw_image.height();
    let mut img = ImageBuffer::new(image_width, image_height);

    println!("image, width: {}, height: {}", image_width, image_height);
    
    for ll in raw_image.layers.iter().rev().enumerate() {
        let layer = ll.1;

        let prop_offsets = layer.properties.iter().find(|l| l.kind == PropertyIdentifier::PropOffsets).unwrap().get_prop_offsets();
        
        let layer_width = layer.width as i32;

        let rgba_buf = &layer.pixels.pixels;

        println!("{}, width: {}, height: {}", layer.name, layer.width, layer.height);
        println!("{}, oWidth: {}, oHeight: {}", layer.name, prop_offsets.width, prop_offsets.height);
        
        for pixel in rgba_buf.iter().enumerate() {
            // layer の透過部分はかきこまない
            if pixel.1.a() == 0 {
                continue;
            }
            
            let rgba = Rgba([pixel.1.r(), pixel.1.g(), pixel.1.b(), pixel.1.a()]);
            let pixel_height = pixel.0 as i32 / layer_width;
            let pixel_width = pixel.0 as i32 - pixel_height * layer_width as i32;
            
            // pixel.0 は単純に index だから、1980, 21 の座標の pixel は 1980 + 100 - 1 で 2000 である
            // そのため x 座標を出すには index - height をすべし

            let offsetted_width = pixel_width + prop_offsets.width;
            let offsetted_height = pixel_height + prop_offsets.height;
            
            if offsetted_width <= 0 || (image_width as i32) <= offsetted_width 
                || offsetted_height <= 0 || (image_height as i32) <= offsetted_height {
                    continue;
                }
            
            img.put_pixel(offsetted_width as u32, offsetted_height as u32, rgba);
        }
    }

    let b64 = general_purpose::STANDARD.encode(img.into_vec());

    return b64;
}
