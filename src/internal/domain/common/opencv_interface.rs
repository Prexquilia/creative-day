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

mod entities;

type Result<T> = opencv::Result<T>;
type Frame = frame::Frame;

trait SourceAdapter {
    pub fn new(source: cvtypes::VideoCapture) -> Self;
    pub fn new_source() -> cvtypes::VideoCapture;
    pub fn new_source_from_file(path: String) -> cvtypes::VideoCapture;

    pub fn is_source_open(&self) -> Result<bool>;
    pub fn grab_frame(&self) -> Result<entities::Frame>;

}

trait ProcessAdapter {
    pub fn new() -> Self;

    pub fn to_greyscale(&self, frame: entities::Frame) -> Result<entities::Frame>;
    pub fn scale_frame(&self, frame: entities::Frame, scale_x:f64, scale_y:f64) -> Result<entities::Frame>;
    pub fn equalize(&self, frame:  entities::Frame) -> Result<entities::Frame>;

    pub fn detect_face(&self, frame: entities::Frame, ops: entities::detect_ops) -> Result<&[entities::area]>;

}

trait WindowAdapter {
    pub fn new() -> Self;

    pub fn flush_frame(&self, frame: entities::Frame) -> Result<()>;

    
}