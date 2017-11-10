mod resource_manager {
    use std::collections::HashMap;
    use cpal;
    use rodio::Sink;

    struct ResourceManager {
        sound_manager: SoundManager,
    }

    struct SoundManager {
        // Sound name to absolute file path
        sound_map: HashMap<String, String>,

        // Currently active track sink
        active_sink: Sink,
    }

    impl SoundManager {
        pub fn get_sound_paths() -> HashMap<String, String> {
            false
        }
    }

    impl ResourceManager {
        pub fn init() {
            let sound_map = SoundManager::get_sound_paths();
            let endpoint = cpal::default_endpoint;
            let mut sm = SoundManager{sound_map, rodio::Sink::new(&endpoint)};
        }
    }
}
