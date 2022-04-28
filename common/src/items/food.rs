use crate::helper::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FoodPellet {
    pub position: Vector2D,
    pub nutrition: u32,
    pub bite_size: u32,
    pub id: usize,
}

impl PartialEq<FoodPellet> for FoodPellet {
    fn eq(&self, other: &FoodPellet) -> bool {
        self.id == other.id
    }
}

impl FoodPellet {
    pub fn get_position(&self) -> Vector2D {
        self.position
    }

    pub fn get_eaten(&mut self) -> u32 {
        if self.nutrition >= self.bite_size {
            self.nutrition -= self.bite_size;
            self.bite_size
        } else {
            self.nutrition
        }
    }

    pub fn is_some_left(&self) -> bool {
        self.nutrition > 0
    }
}
