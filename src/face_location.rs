use std;
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

struct face_time {
    face: Rect,
    contiguous_time: u64,
}

fn square(x: i32) -> i32 {
    x*x
}

pub fn is_previous_face(face : face_time, threshold : i32, previous_face: face_time) -> bool {
    square(face.face.x - previous_face.face.x) < square(threshold) &&
    square(face.face.y - previous_face.face.y) < square(threshold)
}

pub fn get_tracked_faces(timed: &mut std::Vec<face_time>, found: &mut types::VectorOfRect) -> std::Vec<face_time> {
    let mut i: i32;
    for tracked in timed.iter() {
        let mut still_tracked = false;

        let mut ii: i32;
        for face in found.iter() {
            if is_previous_face(tracked, 10, face) {
                still_tracked = true;
                found.remove(ii)
                break;
            }
            ii++;
        }

        if ! still_tracked {
            timed.remove(i)
        }
        i++;
    }

    for face in found.iter() {
        timed.push(face_time{
            face: face,
            contiguous_time: chronos::now(),
        })
    }

    filter_old_faces(*timed)
}

fn filter_old_faces(faces: std::Vec<face_time>) -> std::Vec<face_time> {
    let mut definative = std::Vec::new<face_time>()
    for face in faces.iter() {
        if face.contiguous_time - chronos::now() > 150 {
            definative.push(face);
        }
    }

    definative
}