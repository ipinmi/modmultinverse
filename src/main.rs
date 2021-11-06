// Defining the modular multiplicative inverse using euclidean algorithm

// The modular multiplicative inverse of an integer a is an integer x such that the product ax is equal to 1 with respect to the modulus m
// a.x ≡ 1 mod m 

// EUCLIDEAN ALGORITHM calculates the greatest common factor between two integers a and b (the largest number that divides them without any remainder)
// The Euclidean Algorithm for finding GCD(a, b) states that: 
// if a = 0 then GCD(a,b)= b, because GCD(0, b) = b, and if b  = 0 then GCD(a,b) = a because GCD(a,0) = a
// for non-zero values, represent a in quotient remainder form i.e. a = b.q + r (q = quotient and r = remainder, a = larger integer and b = smaller integer)
// Find GCD(b,r) until the final GCD gives a value of 0 as the remainder since GCD(a,b) = GCD(b,r)

// using EXTENDED euclidean algorithm, we can calcluate the modular multiplicative inverse (MMI) 
// where the multiplicative inverse of “a modulo m” exists if and only if gcd(a, m) = 1)
// the equation becomes ax + by = GCD(a, b) , putting b = m
// ax + my = GCD(a,m) = 1; ax + my ≡ 1 (mod m), 
// removing the second term, we get  a.x ≡ 1 mod m which equals to the modular multiplicative inverse of an integer formula


// The function takes in the following variables, a As the integer, x as the modular multiplicative inverse of the integer and m as the modulo 
// whole numbers can either be represented as signed integers as i32 or i64, 
// isize will be used to create a memory address for these values which would be "pointed" to/ referenced to later.

fn mulinv(a0: i32, m0: i32) -> i32 {
    // if the modulo is 1 then the modular multiplicative inverse (MMI) of the integer will be equal to one. 
    if m0 == 1 { return 1 } // Rust ternary operator 

    // declaring the variables to be mutable to allow for value changes 
    let mut a = a0;
    let mut m = m0;
    let mut y = 0;
    let mut mmi = 1; // mmi represents x in  a.x ≡ 1 mod m
    
    // a must be greater to calculate the MMI since the gcd(a, m) = 1
    while a > 1 {
        let q = a / m ; 
        // to get the new values of x till we have a remainder of 1
        mmi -= q * y ; 
        
        // the remainder becomes the modul0 (m) according to Euclidean algorithm
        a = a % m;
        
        //calculating the MMI involves replacing the b as the new a and r as the new b and calculating the new remainder value
        //in "a = b.q + r" till you achieve r as 1 
        // also recall that b = m in extended euclidean algorithm
        
        // rust uses these modules which work on a memory level to reference the pointer-sized integers (a,m) declared above
        // swap changes two values at mutable locations while maintaining the initialization
        // &mut allow the variables to be referenced and modified
        
        // interchanges the value of a and m [the remainder becomes m] until m becomes 1
        std::mem::swap(&mut a, &mut m);
        
        // interchanges the value of y and mmi until the final mmi with m = 1 is gotten 
        std::mem::swap(&mut y, &mut mmi);
        
    }
    
    // for negative values of mmi, the modulo is added to get the positive equivalent 
    if mmi < 0 { mmi += m0}
    mmi
} 


fn main() {
    println!("Enter a number:");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).unwrap();

    println!("Enter the modulus:");
    let mut modulus = String::new();
    std::io::stdin().read_line(&mut modulus).unwrap();

    //converting the string value to an integer 
    let integer: i32 = number.trim().parse().unwrap();
    let modulo: i32 = modulus.trim().parse().unwrap();

    let modular_inverse = mulinv(integer, modulo);

    println!("The modular multiplicative inverse of {} modulo {} is {}", integer, modulo, modular_inverse);
}