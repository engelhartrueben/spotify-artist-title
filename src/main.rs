use std::process::{Command, Output};

#[cfg(debug_assertions)]
macro_rules! debug {
    ($x:expr) => {
        dbg!($x)
    };
}

fn main() {
    let mut artist = "";
    let mut title = "";

    let meta_data_utf8 = get_meta_data();

    // check if meta data is okay, if NOT, return
    let meta_data_utf8 = match meta_data_utf8.is_ok() {
        true => meta_data_utf8.unwrap(),
        false => {
            debug!(
                meta_data_utf8
                    .err()
                    .inspect(|err| println!("ERROR: {}", err))
            );

            // if error, print empty new line to remove
            // module in polybar
            println!("");
            return;
        }
    };

    // Check if stderr is populated, if so, return
    if !meta_data_utf8.stderr.is_empty() {
        debug!(println!(
            "STDERR: {}",
            String::from_utf8(meta_data_utf8.stderr).unwrap()
        ));

        // if error, print empty new line to remove
        // moduele in polybar
        println!("");
        return;
    }

    // Check if stdout is empty, if so, return
    if meta_data_utf8.stdout.is_empty() {
        debug!(println!("Nothing in stdout!"));

        // if nothing in stdout, print empty new line to remove
        // the polybar module
        println!("");
        return;
    }

    // Safe unwrap
    let meta_data = String::from_utf8(meta_data_utf8.stdout).unwrap();

    // Split lines
    let iter_meta_data: Vec<&str> = meta_data.trim().split('\n').collect();

    // Gross iteration, but it the information is in a relative location
    for (i, e) in (&iter_meta_data).into_iter().enumerate() {
        if e.contains("title") {
            title = &iter_meta_data[i + 1]
                .trim()
                .split("\"")
                .collect::<Vec<&str>>()[1];
        }

        if e.contains("artist") {
            artist = &iter_meta_data[i + 2]
                .trim()
                .split("\"")
                .collect::<Vec<&str>>()[1];
        }
    }

    // Check if either are blank, if so, return
    match title != "" && artist != "" {
        true => println!("{} - by {}", title, artist),
        // if for some reason either are blank
        // print empty new line to remove the polybar module
        false => {
            debug!(println!(
                "Empty title: {}\nEmpty artist: {}",
                title.is_empty(),
                artist.is_empty()
            ));

            println!("");
        }
    };
}

fn get_meta_data() -> Result<Output, std::io::Error> {
    // THANK YOU: https://askubuntu.com/questions/852366/
    let title_cmd = "dbus-send";
    let title_args = "--print-reply --dest=org.mpris.MediaPlayer2.spotify \
                      /org/mpris/MediaPlayer2 org.freedesktop.DBus.Properties.Get \
                      string:org.mpris.MediaPlayer2.Player string:Metadata";

    let res = Command::new(title_cmd).args(title_args.split(" ")).output();

    match res {
        Ok(ok) => Ok(ok),
        Err(error) => Err(error),
    }
}
