use lrc::Lyrics;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. get song data from YesPlayMusic Web API
    let song = reqwest::get("http://127.0.0.1:27232/player")
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    // 2. get id and progress of current song
    let song_id = &song["currentTrack"]["id"].as_i64().unwrap();
    let song_progress = (song["progress"].as_f64().unwrap() * 1000.0 + 1.5) as i64;

    // 3. get lyrics of current song
    let lyrics = reqwest::get(format!("http://127.0.0.1:10754/lyric?id={}", song_id))
        .await?
        .json::<serde_json::Value>()
        .await?;

    // 4. get lyrics data string
    let lyrics_data = lyrics["lrc"]["lyric"].as_str().unwrap();
    let re = Regex::new(r".(\d{2})\d{1}]").unwrap();
    let fixed_lyrics_data = re.replace_all(lyrics_data, ".$1]");
    let lyrics_obj = Lyrics::from_str(fixed_lyrics_data).unwrap();

    // 5. get a matched lyric
    if let Some(index) = lyrics_obj.find_timed_line_index(song_progress) {
        let timed_lines = lyrics_obj.get_timed_lines();
        let lyric = &timed_lines[index];
        println!("{}", lyric.1);
    }

    Ok(())
}
