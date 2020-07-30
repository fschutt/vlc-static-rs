extern crate vlc_static as vlc;
extern crate fltk;

use fltk::*;
use vlc::*;

#[derive(Copy, Clone)]
pub enum Message {
    Play,
    Stop,
}

fn main() {

    let args: Vec<String> = std::env::args().collect();
    let file_to_play = match args.get(1) {
        Some(s) => s,
        None => {
            println!("no input file specified, quitting!"); return;
        },
    };

    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = window::Window::new(100, 100, 800, 600, "Media Player");

    // Create inner window to act as embedded media player
    let mut vlc_win = window::Window::new(10, 10, 780, 520, "");
    vlc_win.end();
    vlc_win.set_color(Color::Black);

    let mut but_play = button::Button::new(320, 545, 80, 40, "Play");
    let mut but_stop = button::Button::new(400, 545, 80, 40, "Stop");

    win.end();
    win.show();
    win.make_resizable(true);

    // Instantiate vlc instance and media player
    let instance = Instance::new().unwrap();
    let md = Media::new_path(&instance, file_to_play).unwrap();
    let mdp = MediaPlayer::new(&instance).unwrap();

    mdp.set_media(&md);

    // Get vlc_win handle that we'll pass to libvlc
    // Linux u32, windows HWND, Mac NSWindow
    let handle = vlc_win.raw_handle();

    // Pass the handle to vlc
    // Method depends on the platform
    // For Linux
    #[cfg(target_os = "linux")]
    mdp.set_xwindow(handle as u32);
    // For Windows
    #[cfg(target_os = "windows")]
    mdp.set_hwnd(handle);
    // For MacOS
    #[cfg(target_os = "macos")]
    mdp.set_nsobject(handle);

    // Disable event handling on vlc's side
    // Do it thru fltk
    mdp.set_key_input(false);
    mdp.set_mouse_input(false);

    let (s, r) = app::channel::<Message>();

    but_play.emit(s, Message::Play);
    but_stop.emit(s, Message::Stop);

    while app.wait().unwrap() {
        match r.recv() {
            Some(val) => match val {
                Message::Play => mdp.play().unwrap(),
                Message::Stop => mdp.stop(),
            },
            None => (),
        }
    }
}