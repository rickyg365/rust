use std::fmt;
use std::ops;


#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    real: f32,
    imaginary: f32
}

impl Complex {
    fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imaginary: -1.0*self.imaginary
        }
    }
}


// Implement Display
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "{} + {}i", self.real, self.imaginary)
    }
}

// Implement + operator
impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        // println!("> Complex[{}+{}i].add(Complex[{}+{}i]) was called", self.real, self.imaginary, other.real, other.imaginary);
        Complex { 
            real: self.real + other.real, 
            imaginary: self.imaginary + other.imaginary 
        }
    }
}

// Implement * operator
impl ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        // FOIL
        let f = self.real*other.real;
        let o = self.real*other.imaginary;
        let i = other.real*self.imaginary;
        let l = self.imaginary*other.imaginary;

        let final_real = f - l;
        let final_imaginary = o + i;

        Self { 
            real: final_real,
            imaginary: final_imaginary
        }
    }
}

impl ops::Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let other_prime = other.conjugate();

        let new_numerator = self*other_prime;
        let new_denomenator = other*other_prime;

        Complex { 
            real: new_numerator.real/new_denomenator.real,
            imaginary: new_numerator.imaginary/new_denomenator.real
        }
    }
}


fn main() {
    // num_data
    let c1:Complex = Complex { real: 2.0, imaginary: 3.0 };
    println!("\nc1 => {}", c1);
    // println!("Debug: {:?}", c1);
    
    let c2:Complex = Complex { real: 1.0, imaginary: 1.0 };
    println!("\nc2 => {}", c2);
    // println!("Debug: {:?}", c2);
    
    let c3:Complex = Complex { real: 2.0, imaginary: 2.0 };
    println!("\nc3 => {}", c3);
    // println!("Debug: {:?}", c3);

    
    // ADDITION
    let c13 = c1 + c3;
    let c23 = c2 + c3;

    // Check if value is still there
    // println!("\nc3: {}", c3);

    assert_eq!(c13, Complex { real: 4.0, imaginary: 5.0 });
    assert_eq!(c23, Complex { real: 3.0, imaginary: 3.0 });

    println!("\nAdd c1 + c3: {}", c13);
    println!("\nAdd c2 + c3: {}", c23);


    // MULTIPLICATION
    let m1 = Complex { real: 3.0, imaginary: 2.0 };
    let m2 = Complex { real: 5.0, imaginary: 6.0 };
    
    let product = m1*m2;

    assert_eq!(product, Complex { real: 3.0, imaginary: 28.0 });
    println!("\nProduct: {}", product);

    // DIVISION
    let d1 = Complex { real: 3.0, imaginary: 2.0 };
    let d2 = Complex { real: 4.0, imaginary: -5.0 };
    
    let quotient = d1/d2;

    assert_eq!(quotient, Complex { real: 0.048780486, imaginary: 0.5609756 });
    println!("\nQuotient: {}", quotient);

}
