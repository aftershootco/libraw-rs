use crate::structs::*;
use core::slice;
use libraw_r::traits::LRString;
use libraw_r::Processor;
use libraw_sys::{
    libraw_canon_makernotes_t, libraw_colordata_t, libraw_data_t, libraw_dng_color_t,
    libraw_dng_levels_t, libraw_fuji_info_t, libraw_image_sizes_t, libraw_imgother_t,
    libraw_iparams_t, libraw_lensinfo_t, libraw_makernotes_t, libraw_nikonlens_t,
    libraw_raw_inset_crop_t, libraw_rawdata_t, NikonLensRadialDistortion,
    NikonLensVignetteCorrection,
};

impl From<&libraw_lensinfo_t> for LibrawLensinfo {
    fn from(data: &libraw_lensinfo_t) -> Self {
        Self {
            libraw_nikonlens: (&data.nikon).into(),
        }
    }
}

impl From<&libraw_nikonlens_t> for LibrawNikonlens {
    fn from(data: &libraw_nikonlens_t) -> Self {
        Self {
            radial_distortion: data.radial_distortion.into(),
            vignette_correction: data.vignette_distortion.into(),
        }
    }
}

impl From<NikonLensRadialDistortion> for NikonLensRadialdistortion {
    fn from(data: NikonLensRadialDistortion) -> Self {
        Self {
            version: data.version.as_ascii().into(),
            on: data.on,
            radial_distortion1: data.radial_distortion1,
            radial_distortion2: data.radial_distortion2,
            radial_distortion3: data.radial_distortion3,
            radial_distortion4: data.radial_distortion4,
        }
    }
}

impl From<NikonLensVignetteCorrection> for NikonLensVignettecorrection {
    fn from(data: NikonLensVignetteCorrection) -> Self {
        Self {
            version: data.version.as_ascii().into(),
            vignette_correction1: data.vignette_correction1,
            vignette_correction2: data.vignette_correction2,
            vignette_correction3: data.vignette_correction3,
            vignette_correction4: data.vignette_correction4,
        }
    }
}

impl From<&libraw_makernotes_t> for LibrawMakernotes {
    fn from(data: &libraw_makernotes_t) -> Self {
        Self {
            libraw_fuji_info: data.fuji.into(),
            libraw_canon_makernotes_t: data.canon.into(),
        }
    }
}

impl From<libraw_fuji_info_t> for LibrawFujiInfo {
    fn from(data: libraw_fuji_info_t) -> Self {
        Self {
            fuji_width: data.fuji_width,
            fuji_layout: data.fuji_layout,
        }
    }
}

impl From<libraw_canon_makernotes_t> for LibrawCanonMakernotes {
    fn from(data: libraw_canon_makernotes_t) -> Self {
        Self {
            focus_distance_lower: data.focus_distance_lower,
            focus_distance_upper: data.focus_distance_upper,
        }
    }
}

impl From<libraw_raw_inset_crop_t> for LibrawRawInsetCrops {
    fn from(value: libraw_raw_inset_crop_t) -> Self {
        Self {
            cleft: value.cleft,
            ctop: value.ctop,
            cwidth: value.cwidth,
            cheight: value.cheight,
        }
    }
}

impl From<&libraw_iparams_t> for LibrawIparams {
    fn from(iparams: &libraw_iparams_t) -> Self {
        Self {
            make: iparams.make.as_ascii().to_string(),
            model: iparams.model.as_ascii().to_string(),
            filters: iparams.filters,
            is_foveon: iparams.is_foveon,
            raw_count: iparams.raw_count,
            dng_version: iparams.dng_version,
            colors: iparams.colors,
            xtrans: iparams.xtrans,
        }
    }
}

impl From<libraw_dng_color_t> for LibrawDngColor {
    fn from(value: libraw_dng_color_t) -> Self {
        Self {
            parsedfields: value.parsedfields,
            illuminant: value.illuminant,
            calibration: value.calibration,
            colormatrix: value.colormatrix,
            forwardmatrix: value.forwardmatrix,
        }
    }
}

impl From<libraw_dng_levels_t> for LibrawDngLevels {
    fn from(value: libraw_dng_levels_t) -> Self {
        Self {
            parsedfields: value.parsedfields,
            dng_cblack: value.dng_cblack.to_vec(),
            dng_black: value.dng_black,
            dng_fcblack: value.dng_fcblack.to_vec(),
            dng_fblack: value.dng_fblack,
            dng_whitelevel: value.dng_whitelevel,
            default_crop: value.default_crop,
            user_crop: value.user_crop,
            preview_colorspace: value.preview_colorspace,
            analogbalance: value.analogbalance,
            asshotneutral: value.asshotneutral,
            baseline_exposure: value.baseline_exposure,
            linear_response_limit: value.LinearResponseLimit,
            shadow_scale: value.shadow_scale,
        }
    }
}

impl From<&libraw_colordata_t> for LibrawColordata {
    fn from(value: &libraw_colordata_t) -> Self {
        Self {
            cblack: value.cblack.to_vec(),
            black: value.black,
            linear_max: value.linear_max,
            maximum: value.maximum,
            cam_mul: value.cam_mul,
            pre_mul: value.pre_mul,
            rgb_cam: value.rgb_cam,
            cam_xyz: value.cam_xyz,
            dng_color: value.dng_color.map(|x| x.into()),
            as_shot_wb_applied: value.as_shot_wb_applied,
            dng_levels: value.dng_levels.into(),
            dng_profile: unsafe {
                if value.dng_profile.is_null() {
                    None
                } else {
                    Some(
                        std::slice::from_raw_parts(
                            value.dng_profile as *const u8,
                            value.dng_profile_len as usize,
                        )
                        .to_owned(),
                    )
                }
            },
        }
    }
}

impl From<&libraw_image_sizes_t> for LibrawImageSizes {
    fn from(libraw_sizes: &libraw_image_sizes_t) -> Self {
        Self {
            raw_height: libraw_sizes.raw_height,
            raw_width: libraw_sizes.raw_width,
            height: libraw_sizes.height,
            width: libraw_sizes.width,
            top_margin: libraw_sizes.top_margin,
            left_margin: libraw_sizes.left_margin,
            iheight: libraw_sizes.iheight,
            iwidth: libraw_sizes.iwidth,
            raw_pitch: libraw_sizes.raw_pitch,
            pixel_aspect: libraw_sizes.pixel_aspect,
            flip: libraw_sizes.flip,
            raw_aspect: libraw_sizes.raw_aspect,
            raw_inset_crops: libraw_sizes.raw_inset_crops.map(|x| x.into()),
        }
    }
}

impl From<&libraw_imgother_t> for LibrawImgother {
    fn from(value: &libraw_imgother_t) -> Self {
        Self {
            iso_speed: value.iso_speed,
            shutter: value.shutter,
            aperture: value.aperture,
            focal_len: value.focal_len,
            timestamp: value.timestamp,
            shot_order: value.shot_order,
            gpsdata: value.gpsdata,
            desc: value.desc.as_ascii().to_string(),
            artist: value.desc.as_ascii().to_string(),
            analogbalance: value.analogbalance,
        }
    }
}

impl From<&libraw_rawdata_t> for LibrawRawdata {
    fn from(value: &libraw_rawdata_t) -> Self {
        let size;
        let data_type: LibrawRawDataType;
        let data = match () {
            _ if !value.raw_image.is_null() => {
                data_type = LibrawRawDataType::RawImage;
                size = value.sizes.raw_width as usize * value.sizes.raw_height as usize;
                let raw_image = unsafe { slice::from_raw_parts(value.raw_image, size) };
                raw_image.iter().map(|i| *i as f32).collect::<Vec<f32>>()
            }
            _ if !value.color3_image.is_null() => {
                data_type = LibrawRawDataType::Color3Image;
                size = value.sizes.raw_width as usize * value.sizes.raw_height as usize * 3;
                let raw_image = unsafe { core::mem::transmute(value.color3_image) };
                let raw_image: &[u16] = unsafe { slice::from_raw_parts(raw_image, size) };
                raw_image.iter().map(|i| *i as f32).collect::<Vec<f32>>()
            }
            _ if !value.color4_image.is_null() => {
                data_type = LibrawRawDataType::Color4Image;
                size = value.sizes.raw_width as usize * value.sizes.raw_height as usize * 4;
                //let mut data = vec![0.0; size];
                let raw_image = unsafe { core::mem::transmute(value.color3_image) };
                let raw_image: &[u16] = unsafe { slice::from_raw_parts(raw_image, size) };
                raw_image.iter().map(|i| *i as f32).collect::<Vec<f32>>()
            }
            _ if !value.float_image.is_null() => {
                data_type = LibrawRawDataType::FloatImage;
                size = value.sizes.raw_width as usize * value.sizes.raw_height as usize;
                let raw_image = unsafe { slice::from_raw_parts(value.float_image, size) };
                raw_image.to_vec()
            }
            _ if !value.float3_image.is_null() => {
                data_type = LibrawRawDataType::Float3Image;
                size = value.sizes.raw_width as usize * value.sizes.raw_height as usize * 3;
                let raw_image = unsafe { core::mem::transmute(value.color3_image) };
                let raw_image = unsafe { slice::from_raw_parts(raw_image, size) };
                raw_image.to_vec()
            }
            _ if !value.float4_image.is_null() => {
                data_type = LibrawRawDataType::Float4Image;
                size = value.sizes.raw_width as usize * value.sizes.raw_height as usize * 4;
                let raw_image = unsafe { core::mem::transmute(value.color4_image) };
                let raw_image = unsafe { slice::from_raw_parts(raw_image, size) };
                raw_image.to_vec()
            }
            _ => {
                return LibrawRawdata {
                    data_type: None,
                    data: None,
                };
            }
        };

        LibrawRawdata {
            data_type: Some(data_type),
            data: Some(data),
        }
    }
}

impl From<Processor> for LibrawData {
    fn from(processor: Processor) -> Self {
        Self::from(&processor)
    }
}

impl From<&Processor> for LibrawData {
    fn from(processor: &Processor) -> Self {
        Self {
            sizes: processor.sizes().into(),
            idata: processor.idata().into(),
            lens: None,
            makernotes: processor.makernotes().into(),
            shootinginfo: None,
            params: None,
            rawparams: None,
            progress_flags: None,
            process_warnings: None,
            color: processor.color().into(),
            other: processor.imgother().into(),
            thumbnail: None,
            thumbs_list: None,
            rawdata: processor.rawdata().into(),
        }
    }
}
