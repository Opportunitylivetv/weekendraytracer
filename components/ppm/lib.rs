use std::io;

/// A PPM image writer based off of the spec found here:
/// http://netpbm.sourceforge.net/doc/ppm.html
pub struct PPMWriter<TWrite> {
    writer: TWrite
}

impl<TWrite: io::Write> PPMWriter<TWrite> {
    pub fn new(writer: TWrite) -> Self {
        PPMWriter {
            writer: writer
        }
    }

    pub fn write(&mut self, image_data: &[f32], width: u32, height: u32) -> io::Result<()> {
        try!(self.write_magic_number());
        try!(self.write_metadata(width, height, 255));

        self.write_image_data(image_data)
    }

    fn write_magic_number(&mut self) -> io::Result<()> {
        write!(&mut self.writer, "P3\n")
    }

    fn write_metadata(&mut self, width: u32, height: u32, max_pixel_val: u32) -> io::Result<()> {
        write!(&mut self.writer, "{} {}\n{}\n", width, height, max_pixel_val)
    }

    fn write_image_data(&mut self, image_data: &[f32]) -> io::Result<()> {
        for pixel in image_data.chunks(3) {
            let r = (255.0 * pixel[0]) as u8;
            let g = (255.0 * pixel[1]) as u8;
            let b = (255.0 * pixel[2]) as u8;

            try!(self.writer.write_all(&[r]));
            try!(self.writer.write_all(&[g]));
            try!(self.writer.write_all(&[b]));
        }

        Ok(())
    }
}

#[test]
fn test_write_ppm() {
    let writer: Vec<u8> = Vec::new();

    let mut ppm_writer = PPMWriter::new(writer);

    let image_data = &[
        1.0, 0.0, 0.0,  0.0, 1.0, 0.0,  0.0, 0.0, 1.0,
        1.0, 1.0, 0.0,  1.0, 1.0, 1.0,  0.0, 0.0, 0.0
    ];

    ppm_writer.write(image_data, 3, 2).unwrap();

    print!("{:?}", ppm_writer.writer);
}
