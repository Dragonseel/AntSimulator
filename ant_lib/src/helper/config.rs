pub struct GeneralConfig {}
impl GeneralConfig {
    pub fn new() -> GeneralConfig {
        GeneralConfig {}
    }
}

pub struct AntConfig {
    pub max_energy: u32,
    pub speed: f32,
    pub angular_speed: f32,
    pub vision_range: f32,
    pub energy_loss: u32,
    pub start_amount: i32,
    pub mouth_reach: f32,
}
impl AntConfig {
    pub fn new() -> AntConfig {
        AntConfig {
            max_energy: 1000,
            speed: 1.0,
            angular_speed: 0.1,
            vision_range: 100.0,
            energy_loss: 1,
            start_amount: 30,
            mouth_reach: 7.0,
        }
    }
}

pub struct FoodConfig {
    pub nutrition: u32,
    pub eaten_value: u32,
    pub spawn_time: i32,
    pub start_amount: i32,
}
impl FoodConfig {
    pub fn new() -> FoodConfig {
        FoodConfig {
            nutrition: 1000,
            eaten_value: 1,
            spawn_time: 100,
            start_amount: 10,
        }
    }
}

pub struct Config {
    pub ants: AntConfig,
    pub food: FoodConfig,
    pub general: GeneralConfig,
}

impl Config {
    pub fn new() -> Config {
        Config {
            ants: AntConfig::new(),
            food: FoodConfig::new(),
            general: GeneralConfig::new(),
        }
    }
}
