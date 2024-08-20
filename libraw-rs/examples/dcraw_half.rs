use libraw_r::{
    traits::LRString, Params, Processor, RawParams, LIBRAW_RAWOPTIONS_DNG_STAGE3_IFPRESENT,
};
use std::path::Path;

pub fn main() -> anyhow::Result<()> {
    for arg in std::env::args().skip(1) {
        let mut p = Processor::builder()
            .with_raw_params([
                RawParams::UseDngSdk(2),
                RawParams::UseRawSpeed(1),
                RawParams::Options(LIBRAW_RAWOPTIONS_DNG_STAGE3_IFPRESENT),
            ])
            .with_params([Params::HalfSize(true)])
            .build();
        p.open(&arg)?;
        println!(
            "Processing {arg} ({}, {})",
            p.idata().make.as_ascii(),
            p.idata().model.as_ascii(),
        );
        p.unpack()?;
        #[cfg(not(feature = "jpeg"))]
        {
            p.dcraw_process()?;
            p.dcraw_ppm_tiff_writer(Path::new(&arg).with_extension("ppm"))?;
            println!("Writing to {arg}.ppm");
        }
        #[cfg(feature = "jpeg")]
        p.to_jpeg_no_rotation(80, None)?;
    }
    Ok(())
}
