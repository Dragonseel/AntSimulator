pub struct GeneralConfig {}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneralConfig {
    pub fn new() -> GeneralConfig {
        GeneralConfig {}
    }
}

pub struct NestConfig {
    pub max_energy: u32,
    pub start_energy: u32,
    pub energy_loss_amount: u32,
    pub energy_loss_rounds: u32,
}

impl Default for NestConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl NestConfig {
    pub fn new() -> NestConfig {
        NestConfig {
            max_energy: 10000,
            start_energy: 4000,
            energy_loss_amount: 1,
            energy_loss_rounds: 5,
        }
    }
}

pub struct AntConfig {
    pub max_energy: u32,
    pub speed: f32,
    pub angular_speed: f32,
    pub vision_range: f32,
    pub energy_loss_amount: u32,
    pub energy_loss_rounds: u32,
    pub mouth_reach: f32,
    pub carry_capacity: u32,
}

impl Default for AntConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl AntConfig {
    pub fn new() -> AntConfig {
        AntConfig {
            max_energy: 1000,
            speed: 1.0,
            angular_speed: 0.1,
            vision_range: 100.0,
            energy_loss_amount: 1,
            energy_loss_rounds: 5,
            mouth_reach: 7.0,
            carry_capacity: 4000,
        }
    }
}

pub struct FoodConfig {
    pub nutrition: u32,
    pub eaten_value: u32,
    pub spawn_time: i32,
    pub start_amount: i32,
}
impl Default for FoodConfig {
    fn default() -> Self {
        Self::new()
    }
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
    pub nests: NestConfig,
    pub general: GeneralConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            ants: AntConfig::new(),
            food: FoodConfig::new(),
            nests: NestConfig::new(),
            general: GeneralConfig::new(),
        }
    }
}
