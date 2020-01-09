pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    alpha: Option<f32>,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, alpha: f32) -> Self {
        Color {
            r: r,
            g: g,
            b: b,
            alpha: Some(alpha),
        }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn alpha(&self) -> Option<f32> {
        self.alpha
    }

    pub fn to_string(&self) -> String {
        match self.alpha {
            None => format!("rgb({}, {}, {})", self.r, self.g, self.b),
            Some(alpha) => format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, alpha),
        }
    }
}

impl From<(u8, u8, u8, f32)> for Color {
    fn from(tu: (u8, u8, u8, f32)) -> Self {
        Color {
            r: tu.0,
            g: tu.1,
            b: tu.2,
            alpha: Some(tu.3),
        }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(tu: (u8, u8, u8)) -> Self {
        Color {
            r: tu.0,
            g: tu.1,
            b: tu.2,
            alpha: None,
        }
    }
}
