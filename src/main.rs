mod complex;
mod piston;

fn main() {
    println!("Hello, world!");

    let mut c1: complex::Complex;
    let c2: complex::Complex;
    let mut c3: complex::Complex;
    let c4: complex::Complex;

    c1 = complex::Complex::new(complex::Form::Cartesian(1.0, 0.0));
    c2 = complex::Complex::new(complex::Form::Cartesian(2.0, 1.0));

    c3 = complex::Complex::new(complex::Form::Polar(1.0, 3.1415));
    c4 = complex::Complex::new(complex::Form::Polar(1.0, 3.1415));

    c1.add(&c2);
    c3.multiply(&c4);

    println!("c1 + c2 = {} + i{}", c1.get_a(), c1.get_b());
    println!("c1 + c2 = {}e^i{}", c1.get_r(), c1.get_t());

    println!("c3 * c4 = {} + i{}", c3.get_a(), c3.get_b());
    println!("c3 * c4 = {}e^i{}", c3.get_r(), c3.get_t());

    let julia_set: complex::Complex;
    julia_set = complex::Complex::new(complex::Form::Cartesian(0.1, -1.0));
    // display_julia_set(&julia_set);
    piston::draw(&julia_set);
}
