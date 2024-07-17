extern crate libraw_r;
extern crate serde;

use std::path::Path;

use libraw_r::structs::LibrawData;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub fn unpack(path: impl AsRef<Path>) -> Result<LibrawData> {
    let mut processor = libraw_r::Processor::builder()
        //.with_params([
        //    libraw_r::Params::HalfSize(true),
        //    libraw_r::Params::UseCameraWb(true),
        //    libraw_r::Params::UserFlip(0),
        //])
        .build();

    processor.open(path)?;
    if processor.inner().image.is_null() {
        processor.unpack()?;
    }

    let data = libraw_r::structs::LibrawData::from(processor);
    Ok(data)
}
