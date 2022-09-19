mod complex;

fn main() {
    println!("Hello, world!");

    let c1: complex::Complex;
    let c2: complex::Complex;

    c1 = complex::Complex::new(complex::Form::Cartesian(1.0, 0.0));
    println!("a= {}, b = {}", c1.get_a(), c1.get_b());
    println!("r= {}, theta = {}", c1.get_r(), c1.get_t());

    c2 = complex::Complex::new(complex::Form::Polar(1.0, 3.1415));
    println!("a= {}, b = {}", c2.get_a(), c2.get_b());
    println!("r= {}, theta = {}", c2.get_r(), c2.get_t());
}
