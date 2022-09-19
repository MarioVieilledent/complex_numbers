pub enum Form {
    Cartesian(f64, f64),
    Polar(f64, f64),
}

pub struct Complex {
    a: f64, // Real part
    b: f64, // Imaginary part
    r: f64, // Distance from origin
    t: f64, // Angle theta
}

impl Complex {
    /**
     * Create a new complex
     */
    pub fn new(form: Form) -> Self {
        match form {
            Form::Cartesian(a, b) => Self {
                a,
                b,
                r: calculate_r(a, b),
                t: calculate_t(a, b),
            },
            Form::Polar(r, t) => Self {
                a: calculate_a(r, t),
                b: calculate_b(r, t),
                r,
                t,
            },
        }
    }

    // Getters
    pub fn get_a(&self) -> f64 {
        self.a
    }
    pub fn get_b(&self) -> f64 {
        self.b
    }
    pub fn get_r(&self) -> f64 {
        self.r
    }
    pub fn get_t(&self) -> f64 {
        self.t
    }
}

/**
 * Given cartesian form of complex number, returns r = |z|
 */
pub fn calculate_r(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

/**
 * Given cartesian form of complex number, returns theta
 */
pub fn calculate_t(a: f64, b: f64) -> f64 {
    (b / a).atan()
}

/**
 * Given exponential form of complex number, returns real part = a
 */
pub fn calculate_a(r: f64, t: f64) -> f64 {
    t.cos() / r
}

/**
 * Given exponential form of complex number, returns imaginary part = b
 */
pub fn calculate_b(r: f64, t: f64) -> f64 {
    t.sin() / r
}
