pub struct _HexBased {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

pub trait _ColorElement {
    fn get_hex_r(&self) -> String;
    fn get_hex_g(&self) -> String;
    fn get_hex_b(&self) -> String;
}

impl _ColorElement for _HexBased {
    fn get_hex_b(&self) -> String {
        return format!("{:x}", self.r);
    }

    fn get_hex_g(&self) -> String {
        return format!("{:x}", self.g);
    }

    fn get_hex_r(&self) -> String {
        return format!("{:x}", self.b);
    }
}
