mod complex;

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
    julia_set = complex::Complex::new(complex::Form::Cartesian(-3.0 / 4.0, 0.0));
    display_julia_set(&julia_set);
}

fn converges(c: &mut complex::Complex, julia_set: &complex::Complex) -> bool {
    let r1: f64 = c.get_r();
    // Julia set formula : Z²+C
    c.square();
    c.add(julia_set);
    c.get_r() < r1
}

fn display_julia_set(julia_set: &complex::Complex) {
    print!(
        "\nJulia set C = {} + i{}\n\n",
        julia_set.get_a(),
        julia_set.get_b()
    );
    let width: isize = 100;
    let height: isize = 35;
    for y in -height..height {
        for x in -width..width {
            let a: f64 = x as f64 / width as f64 * 0.9;
            let b: f64 = y as f64 / height as f64 * 1.6;
            let mut c = complex::Complex::new(complex::Form::Cartesian(a, b));
            if y == 0 {
                if x == 0 {
                    print!("+");
                } else {
                    print!("-");
                }
            } else if x == 0 {
                print!("|");
            } else {
                if converges(&mut c, &julia_set) {
                    print!("&");
                } else {
                    print!(" ");
                }
            }
        }
        print!("\n");
    }
}
