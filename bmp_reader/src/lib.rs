pub struct BMPData {
    width: u32,
    height: u32,
    size: u32,
    colors_count: u32,
    flipped: bool,
    pixels: Vec<u32>,
}

pub fn read(path: &str) -> Result<BMPData, std::io::Error> {
    let data = std::fs::read(path)?;
    let ident = &data[..2];
    if ident != [0x42, 0x4D] {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file format. Not a bmp"))
    }
    else {
        let mut size: u32 = u32::from_le_bytes(data[2..4].try_into().unwrap_or_default());
        let mut raw_size_40: u32 = 0;
        let (mut start, mut header_size): (u32, u32) = (0, 0);
        let (mut width, mut height, mut bits_per_pixel): (u32, i32, u16);
        let mut compression = 0;
        let mut colors_count = u32::MAX;
        match size {
            12 => {
                start = u32::from_le_bytes(data[10..4].try_into().unwrap_or_default());
                header_size = u32::from_le_bytes(data[14..4].try_into().unwrap_or_default());
                width = u16::from_le_bytes(data[18..2].try_into().unwrap()).try_into().unwrap_or_default();
                height = i16::from_le_bytes(data[20..2].try_into().unwrap()).try_into().unwrap_or_default();
        // color planes ignored
                bits_per_pixel = u16::from_le_bytes(data[24..2].try_into().unwrap());
            }
            64 => {
                start = u32::from_le_bytes(data[10..4].try_into().unwrap_or_default());
                header_size = u32::from_le_bytes(data[14..4].try_into().unwrap_or_default());
                width = u16::from_le_bytes(data[18..2].try_into().unwrap()).try_into().unwrap_or_default();
                height = i16::from_le_bytes(data[20..2].try_into().unwrap()).try_into().unwrap_or_default();
        // color planes ignored
                bits_per_pixel = u16::from_le_bytes(data[24..2].try_into().unwrap_or_default());
            }
            40 => {
                start = u32::from_le_bytes(data[10..4].try_into().unwrap_or_default());
                header_size = u32::from_le_bytes(data[14..4].try_into().unwrap_or_default());
                width = u32::from_le_bytes(data[18..4].try_into().unwrap_or_default());
                height = i32::from_le_bytes(data[22..4].try_into().unwrap_or_default());
                // color planes ignored
                bits_per_pixel = u16::from_le_bytes(data[28..2].try_into().unwrap_or_default());
                compression = u32::from_le_bytes(data[30..4].try_into().unwrap_or_default());
                if compression != 0 {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Incompatible compression type!"));
                }
                raw_size_40 = u32::from_le_bytes(data[34..4].try_into().unwrap_or_default());
                if raw_size_40 != size {
                    print!("Raw size of 40 header is not the same as raw size of bitmap metadata. Validity under doubt.");
                }
                colors_count = u32::from_le_bytes(data[50..5].try_into().unwrap_or_default());
            }
            _ => {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unsupported bitmap header"));
            }
        }
        dbg!("BMP file content start: {}", start);
        let mut flpd = false;
        if height < 0 {
            flpd = true;
            height = height * -1;
        }
        let height = height as u32;
        let pixels_slice = &data[start as usize..size as usize];
        let row_size: u32 = ((u32::from(bits_per_pixel) * width) as f32 / 32.0).ceil() as u32 * 4;
        let pixel_array_size = row_size * height;
        if pixel_array_size != size {
            print!("Pixel array size doesn't match bitmap metadata size! Validity under doubt.");
        }
        let pixels: Vec<u32>= match bits_per_pixel {
            1 => {
                let mut pl = Vec::<u32>::new();
                for p in pixels_slice {
                    pl.push(u32::from(p >> 7));
                    pl.push(u32::from(p << 1 >> 7));
                    pl.push(u32::from(p << 2 >> 7));
                    pl.push(u32::from(p << 3 >> 7));
                    pl.push(u32::from(p << 4 >> 7));
                    pl.push(u32::from(p << 5 >> 7));
                    pl.push(u32::from(p << 6 >> 7));
                    pl.push(u32::from(p << 7 >> 7));
                }
                pl
            },
            2 => {
                let mut pl = Vec::<u32>::new();
                for p in pixels_slice {
                    pl.push(u32::from(p >> 6));
                    pl.push(u32::from(p << 2 >> 6));
                    pl.push(u32::from(p << 4 >> 6));
                    pl.push(u32::from(p << 6 >> 6));
                }
                pl
            }
            4 => {
                let mut pl = Vec::<u32>::new();
                for p in pixels_slice {
                    pl.push(u32::from(p >> 4));
                    pl.push(u32::from(p << 4 >> 4));
                }
                pl
            }
            8 => {
                let mut pl = Vec::<u32>::new();
                for p in pixels_slice {
                    pl.push(u32::from(p.clone()));
                }
                pl
            }
            16 => {
                let mut pl = Vec::<u32>::new();
                let iter = pixels_slice.iter();
                for i in  (0..iter.count()).step_by(2) {
                    pl.push(u32::from(u16::from_le_bytes([pixels_slice[i], pixels_slice[i + 1]])));
                }
                pl
            },
            /*24 => {
                let mut pl = Vec::<u32>::new();
                let iter = pixels_slice.iter();
                for i in  (0..iter.count()).step_by(3) {
                    pl.push(u32::from(u24::from_le_bytes([pixels_slice[i], pixels_slice[i + 1], pixels_slice[i + 2]])));
                }
                pl
            },*/
            32 => {
                let mut pl = Vec::<u32>::new();
                let iter = pixels_slice.iter();
                for i in  (0..iter.count()).step_by(4) {
                    pl.push(u32::from_le_bytes([pixels_slice[i], pixels_slice[i + 1], pixels_slice[i + 2], pixels_slice[i + 3]]));
                }
                pl
            }
            _ => {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Unsupported bits per pixel value"));
            }
        };
        Ok(BMPData {
            width: width,
            height: height,
            size: size,
            colors_count: colors_count,
            flipped: flpd,
            pixels: pixels,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
