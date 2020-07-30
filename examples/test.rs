extern crate vlc_static;

fn main() {
    println!("{}", unsafe { vlc_static::sys::libvlc_delay(0) });
}