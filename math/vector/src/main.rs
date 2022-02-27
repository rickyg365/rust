use std::fmt;
use std::ops;


#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2D {
    i: f32,
    j: f32
}

impl Vector2D {
    fn conjugate(&self) -> Vector2D {
        Vector2D {
            i: self.i,
            j: -1.0*self.j
        }
    }
}


// Implement Display
impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "{}i + {}j", self.i, self.j)
    }
}

// Implement + operator
impl ops::Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        // println!("> Complex[{}+{}i].add(Complex[{}+{}i]) was called", self.real, self.imaginary, other.real, other.imaginary);
        Vector2D { 
            i: self.real + other.real, 
            j: self.imaginary + other.imaginary 
        }
    }
}

// Implement * operator
impl ops::Mul<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn mul(self, other: Vector2D) -> Vector2D {
        // FOIL
        let f = self.i*other.i;
        let o = self.i*other.j;
        let i = other.i*self.j;
        let l = self.j*other.j;

        let final_i = f - l;
        let final_j = o + i;

        Self { 
            i: final_i,
            j: final_j
        }
    }
}

impl ops::Div<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn div(self, other: Vector2D) -> Vector2D {
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
