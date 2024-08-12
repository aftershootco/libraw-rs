use core::slice;

use crate::{traits::LRString, Processor};

use libraw_sys::{
    libraw_canon_makernotes_t, libraw_colordata_t, libraw_data_t, libraw_dng_color_t, libraw_dng_levels_t, libraw_fuji_info_t, libraw_image_sizes_t, libraw_imgother_t, libraw_iparams_t, libraw_lensinfo_t, libraw_makernotes_t, libraw_nikonlens_t, libraw_raw_inset_crop_t, libraw_rawdata_t, NikonLensRadialDistortion, NikonLensVignetteCorrection
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawIparams {
    pub make: String,
    pub model: String,
    pub filters: u32,
    pub is_foveon: u32,
    pub raw_count: u32,
    pub dng_version: u32,
    pub colors: i32,
    pub xtrans: [[i8; 6]; 6],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawImageSizes {
    pub raw_height: u16,
    pub raw_width: u16,
    pub height: u16,
    pub width: u16,
    pub top_margin: u16,
    pub left_margin: u16,
    pub iheight: u16,
    pub iwidth: u16,
    pub raw_pitch: u32,
    pub pixel_aspect: f64,
    pub flip: i32,
    pub raw_aspect: u16,
    pub raw_inset_crops: [LibrawRawInsetCrops; 2usize],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawRawInsetCrops {
    pub cleft: u16,
    pub ctop: u16,
    pub cwidth: u16,
    pub cheight: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawLensinfo {
    libraw_nikonlens: LibrawNikonlens,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawNikonlens {
    radial_distortion: NikonLensRadialdistortion,
    vignette_correction: NikonLensVignettecorrection,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NikonLensRadialdistortion {
    pub version: String,
    pub on: u8,
    pub radial_distortion1: f32,
    pub radial_distortion2: f32,
    pub radial_distortion3: f32,
    pub radial_distortion4: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NikonLensVignettecorrection {
    pub version: String,
    pub vignette_correction1: f32,
    pub vignette_correction2: f32,
    pub vignette_correction3: f32,
    pub vignette_correction4: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawMakernotes {
    libraw_fuji_info: LibrawFujiInfo,
    libraw_canon_makernotes_t: LibrawCanonMakernotes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawCanonMakernotes {
    focus_distance_lower: f32,
    focus_distance_upper: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawFujiInfo {
    fuji_width: u16,
    fuji_layout: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawShootinginfo {}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawOutputParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawRawUnpackParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawDngColor {
    pub parsedfields: u32,
    pub illuminant: u16,
    pub calibration: [[f32; 4usize]; 4usize],
    pub colormatrix: [[f32; 3usize]; 4usize],
    pub forwardmatrix: [[f32; 4usize]; 3usize],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawDngLevels {
    pub parsedfields: u32,
    pub dng_cblack: Vec<u32>,
    pub dng_black: u32,
    pub dng_fcblack: Vec<f32>,
    pub dng_fblack: f32,
    pub dng_whitelevel: [u32; 4usize],
    pub default_crop: [u16; 4usize],
    pub user_crop: [f32; 4usize],
    pub preview_colorspace: u32,
    pub analogbalance: [f32; 4usize],
    pub asshotneutral: [f32; 4usize],
    pub baseline_exposure: f32,
    pub linear_response_limit: f32,
    pub shadow_scale: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawColordata {
    pub cblack: Vec<u32>,
    pub black: u32,
    pub linear_max: [u32; 4usize],
    pub maximum: u32,
    pub cam_mul: [f32; 4usize],
    pub pre_mul: [f32; 4usize],
    pub rgb_cam: [[f32; 4usize]; 3usize],
    pub cam_xyz: [[f32; 3usize]; 4usize],
    pub dng_color: [LibrawDngColor; 2usize],
    pub as_shot_wb_applied: i32,
    pub dng_levels: LibrawDngLevels,
    pub dng_profile: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawImgother {
    pub iso_speed: f32,
    pub shutter: f32,
    pub aperture: f32,
    pub focal_len: f32,
    pub timestamp: i64,
    pub shot_order: u32,
    pub gpsdata: [u32; 32usize],
    pub desc: String,
    pub artist: String,
    pub analogbalance: [f32; 4usize],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawThumbnail {}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawThumbnailList {}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawRawdata {
    pub data_type: Option<LibrawRawDataType>,
    pub data: Option<Vec<f32>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LibrawRawDataType {
    RawImage,
    Color4Image,
    Color3Image,
    FloatImage,
    Float3Image,
    Float4Image,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibrawData {
    pub sizes: Option<LibrawImageSizes>,
    pub idata: Option<LibrawIparams>,
    pub lens: Option<LibrawLensinfo>,
    pub makernotes: Option<LibrawMakernotes>,
    pub shootinginfo: Option<LibrawShootinginfo>,
    pub params: Option<LibrawOutputParams>,
    pub rawparams: Option<LibrawRawUnpackParams>,
    pub progress_flags: Option<u32>,
    pub process_warnings: Option<u32>,
    pub color: Option<LibrawColordata>,
    pub other: Option<LibrawImgother>,
    pub thumbnail: Option<LibrawThumbnail>,
    pub thumbs_list: Option<LibrawThumbnailList>,
    pub rawdata: Option<LibrawRawdata>,
}

impl From<&libraw_lensinfo_t> for LibrawLensinfo  {
    fn from(data: &libraw_lensinfo_t) -> Self {
        Self {
            libraw_nikonlens: (&data.nikon).into(),
        }
    }
}

impl From<&libraw_nikonlens_t> for LibrawNikonlens {
    fn from(data: &libraw_nikonlens_t) -> Self {
        Self { radial_distortion: data.radial_distortion.into(), vignette_correction: data.vignette_distortion.into() }
    }
}

impl From<NikonLensRadialDistortion> for NikonLensRadialdistortion {
    fn from(data: NikonLensRadialDistortion) -> Self {
        Self { version: data.version.as_ascii().into(), on: data.on, radial_distortion1: data.radial_distortion1, radial_distortion2: data.radial_distortion2, radial_distortion3: data.radial_distortion3, radial_distortion4: data.radial_distortion4 }
    }
}

impl From<NikonLensVignetteCorrection> for NikonLensVignettecorrection {
    fn from(data: NikonLensVignetteCorrection) -> Self {
        Self { version: data.version.as_ascii().into(), vignette_correction1: data.vignette_correction1, vignette_correction2: data.vignette_correction2, vignette_correction3: data.vignette_correction3, vignette_correction4: data.vignette_correction4 }
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
                    Some(std::slice::from_raw_parts(value.dng_profile as *const u8, value.dng_profile_len as usize).to_owned())
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
        Self {
            sizes: Some(processor.sizes().into()),
            idata: Some(processor.idata().into()),
            lens: None,
            makernotes: Some(processor.makernotes().into()),
            shootinginfo: None,
            params: None,
            rawparams: None,
            progress_flags: None,
            process_warnings: None,
            color: Some(processor.color().into()),
            other: Some(processor.imgother().into()),
            thumbnail: None,
            thumbs_list: None,
            rawdata: Some(processor.rawdata().into()),
        }
    }
}

impl From<&Processor> for LibrawData {
    fn from(processor: &Processor) -> Self {
        Self {
            sizes: Some(processor.sizes().into()),
            idata: Some(processor.idata().into()),
            lens: None,
            makernotes: None,
            shootinginfo: None,
            params: None,
            rawparams: None,
            progress_flags: None,
            process_warnings: None,
            color: Some(processor.color().into()),
            other: Some(processor.imgother().into()),
            thumbnail: None,
            thumbs_list: None,
            rawdata: Some(processor.rawdata().into()),
        }
    }
}
