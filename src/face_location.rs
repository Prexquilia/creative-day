use opencv::core::{Rect, Scalar, Size};
use opencv::{highgui, imgproc, objdetect, prelude::*, types};

pub fn is_face_in_box(face: Rect, area: Rect) -> bool {
    face.x > area.x &&
    face.y > area.y &&
    face.y + face.height < area.y + area.height &&
    face.x + face.width < area.x + area.width
}

pub fn is_face_clipping_box(face: Rect, area: Rect) -> bool {
    face.x > area.x && face.x < area.x + area.width ||
    face.y > area.y && face.y < area.y + area.height || 
    face.x + face.width > area.x && face.x + face.width < area.x + area.width ||
    face.y + face.height > area.y && face.y + face.height < area.y +area.height
}