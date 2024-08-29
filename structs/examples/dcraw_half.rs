#[cfg(feature = "libraw")]
use libraw_r::Processor;
#[cfg(feature = "libraw")]
use libraw_structs::LibrawData;
#[cfg(feature = "libraw")]
use std::error::Error;

pub fn main() {
    #[cfg(feature = "libraw")]
    {
        let path = std::env::args().skip(1).next().unwrap();

        let mut proc = Processor::builder().build();
        proc.open(path).unwrap();
        if proc.inner().image.is_null() {
            proc.unpack().unwrap();
        }
        let structure = LibrawData::from(proc);
        println!("{:?}", structure.rawdata.data_type);
    }
}
