use libraw_r::{traits::LRString, Params, Processor, RawParams};
use std::path::Path;

pub fn main() -> anyhow::Result<()> {
    for arg in std::env::args().skip(1) {
        let mut p = Processor::builder()
            .with_raw_params([RawParams::UseDngSdk(2)])
            .with_params([Params::HalfSize(false)])
            .build();
        //let mut p = libraw_r::defaults::half_size();
        p.open(&arg)?;
        dbg!(p.params());
        println!(
            "Processing {arg} ({}, {})",
            p.idata().make.as_ascii(),
            p.idata().model.as_ascii(),
        );
        let img = p.to_jpeg_no_rotation(100)?;
        std::fs::write("./result.jpeg", img);
        //p.dcraw_process()?;
        //p.dcraw_ppm_tiff_writer(Path::new(&arg).with_extension("ppm"))?;
        //println!("Writing to {arg}.ppm");
    }
    Ok(())
}
