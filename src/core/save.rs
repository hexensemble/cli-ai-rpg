use serde::Serialize;
use serde_json;
use std::fs;
use std::path::PathBuf;

// Struct for Save Game Manager
pub struct SaveGameManager {
    save_data: SaveData,
}

// Functions for Save Game Manager
impl SaveGameManager {
    // Create a new Save Game Manager
    pub fn new() -> Self {
        Self {
            save_data: SaveData::new(),
        }
    }

    // Save the game to JSON
    pub fn save(
        &mut self,
        world_manager: &crate::world::manager::WorldManager,
        time_manager: &crate::world::time::TimeManger,
        weather_manager: &crate::world::weather::WeatherManager,
    ) -> Result<(), std::io::Error> {
        // Save player
        if let Some(player) = &world_manager.player {
            self.save_data.player = Some(player.clone());
        } else {
            eprint!("Player not initialized")
        }

        // Save time
        self.save_data.time = time_manager
            .time_arc_rwlock
            .as_ref()
            .and_then(|game_time| game_time.read().ok().map(|t| t.clone()));

        if self.save_data.time.is_none() {
            eprintln!("GameTime not initialized");
        }

        // Save weather
        self.save_data.weather = weather_manager
            .weather_arc_rwlock
            .as_ref()
            .and_then(|game_weather| game_weather.read().ok().map(|w| w.clone()));

        if self.save_data.weather.is_none() {
            eprint!("GameWeather not initialized");
        }

        // Serialize JSON
        let json = serde_json::to_string_pretty(&self.save_data)?;

        // Path to save JSON file
        let save_path = PathBuf::from("saves").join("save.json");

        // Check directory exists and create if it doesn't
        if let Some(parent) = save_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write to file
        fs::write(&save_path, json)?;

        Ok(())
    }
}

// Struct for Save Data
#[derive(Serialize)]
struct SaveData {
    player: Option<crate::entities::player::Player>,
    time: Option<crate::world::time::GameTime>,
    weather: Option<crate::world::weather::GameWeather>,
}

// Functions for Save Data
impl SaveData {
    // Create a new Save Data
    fn new() -> Self {
        Self {
            player: None,
            time: None,
            weather: None,
        }
    }
}
