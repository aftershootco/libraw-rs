use libraw_r::traits::LRString;
use std::{io::Seek, path::Path};

pub fn main() -> anyhow::Result<()> {
    for arg in std::env::args().skip(1) {
        // let mut p = Processor::builder()
        //     .with_params([Params::HalfSize(true)])
        //     .build();
        let p = libraw_r::EmptyProcessor::new()?;
        let file = std::fs::File::open(&arg)?;
        let myfile = MyFile { file };
        let mut buffered = std::io::BufReader::new(myfile);
        let mut p = p.open(&mut buffered)?;
        println!(
            "Processing {arg} ({}, {})",
            p.idata().make.as_ascii(),
            p.idata().model.as_ascii(),
        );
        p.unpack()?;
        p.dcraw_process()?;
        p.dcraw_ppm_tiff_writer(Path::new(&arg).with_extension("ppm"))?;
        // drop(buffered);
        println!("Writing to {arg}.ppm");
    }
    Ok(())
}

pub struct MyFile {
    file: std::fs::File,
}

impl core::fmt::Debug for MyFile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MyFile").finish()
    }
}

impl std::io::Read for MyFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buf)
    }
}

impl std::io::Seek for MyFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.file.seek(pos)
    }
}

impl Drop for MyFile {
    fn drop(&mut self) {
        println!("Dropping MyFile");
        panic!()
    }
}
