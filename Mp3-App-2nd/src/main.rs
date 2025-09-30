use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;
use std::thread;

use rodio::{Decoder, OutputStream, Sink};
use crossterm::event::{self, Event, KeyCode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let music_dir = PathBuf::from(r"C:\Users\Admin\Music");

    
    let mut playlist: Vec<_> = fs::read_dir(&music_dir)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().map(|ext| ext == "mp3").unwrap_or(false))
        .collect();

    if playlist.is_empty() {
        println!("No mp3 files found in {:?}", music_dir);
        return Ok(());
    }

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let mut current = 0;

    let mut sink = play_song(&playlist[current], &stream_handle)?;

    println!("Controls: [Space] Pause/Resume | [N] Next | [Q] Quit");

    loop {
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(' ') => {
                        if sink.is_paused() {
                            sink.play();
                            println!("Resumed");
                        } else {
                            sink.pause();
                            println!("Paused");
                        }
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        current = (current + 1) % playlist.len();
                        sink.stop();
                        sink = play_song(&playlist[current], &stream_handle)?;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        println!("Exiting...");
                        break;
                    }
                    _ => {}
                }
            }
        }

        if sink.empty() {
            
            current = (current + 1) % playlist.len();
            sink = play_song(&playlist[current], &stream_handle)?;
        }

        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

fn play_song(path: &PathBuf, stream_handle: &rodio::OutputStreamHandle) -> Result<Sink, Box<dyn std::error::Error>> {
    println!("Now playing: {:?}", path.file_name().unwrap());

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let sink = Sink::try_new(stream_handle)?;
    let source = Decoder::new_mp3(reader)?;
    sink.append(source);

    Ok(sink)
}
