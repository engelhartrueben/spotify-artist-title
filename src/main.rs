use std::process::{Command, Output};

fn main() {
    let meta_data_utf8 = get_meta_data();

    // check if meta data is okay, if NOT, return
    let meta_data_utf8 = match meta_data_utf8.is_ok() {
        true => meta_data_utf8.unwrap(),
        false => {
            debug_assert!({
                meta_data_utf8
                    .err()
                    .inspect(|err| println!("ERROR: {}", err));
                true
            });

            // if error, print empty new line to remove
            // module in polybar
            println!("");
            return;
        }
    };

    // Check if stderr is populated, if so, return
    if !meta_data_utf8.stderr.is_empty() {
        debug_assert!({
            println!(
                "STDERR: {}",
                String::from_utf8(meta_data_utf8.stderr).unwrap()
            );
            true
        });

        // if error, print empty new line to remove
        // moduele in polybar
        println!("");
        return;
    }

    // Check if stdout is empty, if so, return
    if meta_data_utf8.stdout.is_empty() {
        debug_assert!({
            println!("Nothing in stdout!");
            true
        });

        // if nothing in stdout, print empty new line to remove
        // the polybar module
        println!("");
        return;
    }

    // Safe unwrap
    let meta_data = String::from_utf8(meta_data_utf8.stdout).unwrap();

    // Split lines
    let iter_meta_data: Vec<&str> = meta_data.trim().split("\"").collect();

    // use absolute position of information
    let title = &iter_meta_data[29].trim();

    let artist = &iter_meta_data[21].trim();

    println!("{} - by {}", title, artist);
}

fn get_meta_data() -> Result<Output, std::io::Error> {
    // THANK YOU: https://askubuntu.com/questions/852366/
    let title_cmd = "dbus-send";
    let title_args = "--print-reply --dest=org.mpris.MediaPlayer2.spotify \
                      /org/mpris/MediaPlayer2 org.freedesktop.DBus.Properties.Get \
                      string:org.mpris.MediaPlayer2.Player string:Metadata";

    Command::new(title_cmd).args(title_args.split(" ")).output()
}
