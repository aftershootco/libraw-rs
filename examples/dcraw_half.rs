use libraw_r::traits::LRString;
use std::path::Path;

pub fn main() -> anyhow::Result<()> {
    for arg in std::env::args().skip(1) {
        // let mut p = Processor::builder()
        //     .with_params([Params::HalfSize(true)])
        //     .build();
        let p = libraw_r::EmptyProcessor::new()?;
        let mut p = p.open(&arg)?;
        println!(
            "Processing {arg} ({}, {})",
            p.idata().make.as_ascii(),
            p.idata().model.as_ascii(),
        );
        p.unpack()?;
        p.dcraw_process()?;
        // p.dcraw_ppm_tiff_writer(Path::new(&arg).with_extension("ppm"))?;
        // println!("Writing to {arg}.ppm");
    }
    Ok(())
}
