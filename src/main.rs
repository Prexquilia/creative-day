

use anyhow::Result; // Automatically handle the error types
use opencv::{
    prelude::*,
    videoio,
    highgui
}; // Note, the namespace of OpenCV is changed (to better or worse). It is no longer one enormous.
fn main() -> Result<(), > {
    // Open a GUI window
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    // Open the web-camera (assuming you have one)
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default(); // This array will store the web-cam data
    // Read the camera
    // and display in the window
    loop {
        cam.read(&mut frame)?;
        highgui::imshow("window", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113 { // quit with q
            break;
        }
    }
    Ok(())
}


/*
struct int(i64);

impl std::fmt::Display for int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl int {
    fn Clone(&self) -> Self {
        int(self.0)
    }
}

fn main() {
    println!("hello world!");
    println!("test {0} {1} {2}", "sup", "{0}", "hi");
    let mut a = 360_i64;
    println!("an integer {0}", a);

    a = 380;
    println!("another integer {0}", a);

    let b = a.clone();
    println!("test {0} {1}", a, b);

    a = 420;
    println!("test {0} {1}", a, b);

    let mut a_custom = int(64);
    println!("custom int {0}", a_custom);

    let b_custom = a_custom.Clone();
    println!("customs {0} {1}", a_custom, b_custom);

    /*
    let b_custom = a_custom;
    a_custom = int(101); 
    println!("moved {0} {1}", a_custom, b_custom)
    */
}
*/