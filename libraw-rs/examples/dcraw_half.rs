use libraw_r::{Params, Processor, RawParams, traits::LRString};

pub fn main() -> anyhow::Result<()> {
    for arg in std::env::args().skip(1) {
        let mut p = Processor::builder()
            .with_raw_params([RawParams::UseDngSdk(2), RawParams::Options(1 << 1 | 1 << 21)])
            .with_params([Params::HalfSize(false)])
            .build();
        //let mut p = libraw_r::defaults::half_size();
        p.open(&arg)?;
        dbg!(p.rawparams());
        println!(
            "Processing {arg} ({}, {})",
            p.idata().make.as_ascii(),
            p.idata().model.as_ascii(),
        );
        let img = p.to_jpeg_no_rotation(100)?;
        std::fs::write("./result.jpeg", img)?;
        //p.unpack()?;
        //p.dcraw_process()?;
        //p.dcraw_ppm_tiff_writer(Path::new(&arg).with_extension("ppm"))?;
        //println!("Writing to {arg}.ppm");
    }
    Ok(())
}
