use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawRawInsetCrops {
    pub cleft: u16,
    pub ctop: u16,
    pub cwidth: u16,
    pub cheight: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawLensinfo {
    pub libraw_nikonlens: LibrawNikonlens,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawNikonlens {
    pub radial_distortion: NikonLensRadialdistortion,
    pub vignette_correction: NikonLensVignettecorrection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NikonLensRadialdistortion {
    pub version: String,
    pub on: u8,
    pub radial_distortion1: f32,
    pub radial_distortion2: f32,
    pub radial_distortion3: f32,
    pub radial_distortion4: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NikonLensVignettecorrection {
    pub version: String,
    pub vignette_correction1: f32,
    pub vignette_correction2: f32,
    pub vignette_correction3: f32,
    pub vignette_correction4: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawMakernotes {
    pub libraw_fuji_info: LibrawFujiInfo,
    pub libraw_canon_makernotes_t: LibrawCanonMakernotes,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawCanonMakernotes {
    pub focus_distance_lower: f32,
    pub focus_distance_upper: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawFujiInfo {
    pub fuji_width: u16,
    pub fuji_layout: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawShootinginfo {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawOutputParams {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawRawUnpackParams {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawDngColor {
    pub parsedfields: u32,
    pub illuminant: u16,
    pub calibration: [[f32; 4usize]; 4usize],
    pub colormatrix: [[f32; 3usize]; 4usize],
    pub forwardmatrix: [[f32; 4usize]; 3usize],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawThumbnail {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawThumbnailList {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawRawdata {
    pub data_type: Option<LibrawRawDataType>,
    pub data: Option<Vec<f32>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum LibrawRawDataType {
    RawImage,
    Color4Image,
    Color3Image,
    FloatImage,
    Float3Image,
    Float4Image,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibrawData {
    pub sizes: LibrawImageSizes,
    pub idata: LibrawIparams,
    pub lens: Option<LibrawLensinfo>,
    pub makernotes: LibrawMakernotes,
    pub shootinginfo: Option<LibrawShootinginfo>,
    pub params: Option<LibrawOutputParams>,
    pub rawparams: Option<LibrawRawUnpackParams>,
    pub progress_flags: Option<u32>,
    pub process_warnings: Option<u32>,
    pub color: LibrawColordata,
    pub other: LibrawImgother,
    pub thumbnail: Option<LibrawThumbnail>,
    pub thumbs_list: Option<LibrawThumbnailList>,
    pub rawdata: LibrawRawdata,
}
