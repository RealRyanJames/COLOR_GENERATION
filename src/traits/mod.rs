use crate::colors::Color;

impl Color {
    pub fn get_first(&self) -> u32 {
        self.0 as u32
    }

    pub fn get_second(&self) -> u32 {
        self.1 as u32
    }

    pub fn get_third(&self) -> u32 {
        self.2 as u32
    }
}
