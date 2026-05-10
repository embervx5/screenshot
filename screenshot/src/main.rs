use screenshots::Screen;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut path = home::home_dir().expect("ERROR: could not find home directory");
    path.push("Documents");
    path.push("Screenshots");
    fs::create_dir_all(&path).expect("ERROR: couldnt create screenshot directory");
    let mut counter = 0;
    let mut final_path: PathBuf;
    loop {
        let filename = if counter == 0 {
            "screenshot.png".to_string()
        } else {
            format!("screenshot_{}.png", counter)
        };
        final_path = path.join(filename);
        if !final_path.exists() {
            break;
        }
        counter += 1;
    }
    let screen = Screen::all().unwrap()[0];
    let image_buffer = screen.capture().unwrap();
    image_buffer.save(&final_path).expect("ERROR: check folder permissions?");
    println!("Screenshot saved to: {:?}", final_path);
}
