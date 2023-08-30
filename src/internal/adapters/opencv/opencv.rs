use std::string::String;
use opencv::{
    vidio,
    highgui,
    imgproc, 
    objdetect, 
    prelude::*, 
    types as cvtypes,
    core::{
        Rect, 
        Scalar, 
        Size
    }
};

type Result<T> = opencv::Result<T>;

trait opencvAdapter {
    pub fn new(source: cvtypes::VideoCapture) -> self;
    pub fn new_source() -> cvtypes::VideoCapture;
    pub fn new_source_from_file(path: String) -> cvtypes::VideoCapture;

    pub fn is_source_open(&self) -> Result<bool>;
    pub fn grab_frame(&self) -> Result<Option<Mat>>;

    pub fn flush_frame(&self, frame: Mat) -> Result<()>;
    
    pub fn to_greyscale(&self, frame: Mat) -> Result<Mat>;
    pub fn scale_frame(&self, frame:Mat, scale_x:f64, scale_y:f64) -> Result<Mat>;
    pub fn equalize(&self, frame:Mat) -> Result<Mat>;

}