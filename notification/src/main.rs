use std::thread;
use std::time::Duration;
use chrono::Local;
extern crate winrt_notification;
use winrt_notification::{IconCrop,Sound, Toast};
use std::path::Path;

fn main() {
    let mut count = 1;

    loop {
        println!("{} Time: {:?}", count, Local::now().format("%H:%M:%S").to_string());
        count += 1;

        Toast::new(Toast::POWERSHELL_APP_ID)
        .icon(
            &Path::new("C:/Users/Palindra/Downloads/gym.png"), 
            IconCrop::Circular,
            "the gym",
        )
        .title("Hour passed")
        .text1("Are you working?")
        .image(&Path::new("C:/Users/Palindra/Downloads/push up.png"), "the gym")
        .text2("Give me 10 push ups!!!")
        .sound(Some(Sound::Reminder))
        .duration(winrt_notification::Duration::Short)
        .show()
        .expect("unable to toast");

        thread::sleep(Duration::from_secs(3595));
    }
}
