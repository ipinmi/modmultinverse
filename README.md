# modmultinverse
A code that calculates the Modular multiplicative inverse of a number using the Euclidean Algorithm in Rust

The modular multiplicative inverse of an integer a is an integer x such that the product ax is equal to 1 with respect to the modulus m
**a.x ≡ 1 mod m**

EUCLIDEAN ALGORITHM calculates the greatest common factor between two integers a and b (the largest number that divides them without any remainder) 
The Euclidean Algorithm for finding GCD(a, b) states that: 
if a = 0 then GCD(a,b)= b, because GCD(0, b) = b,
and if b  = 0 then GCD(a,b) = a because GCD(a,0) = a

For non-zero values, represent a in quotient remainder form i.e. a = b.q + r (q = quotient and r = remainder, a = larger integer and b = smaller integer) 
Find GCD(b,r) until the final GCD gives a value of 0 as the remainder since GCD(a,b) = GCD(b,r)

using EXTENDED euclidean algorithm, we can calcluate the modular multiplicative inverse (MMI) where the multiplicative inverse of “a modulo m” exists if and only if gcd(a, m) = 1)
the equation becomes ax + by = GCD(a, b) , putting b = m
ax + my = GCD(a,m) = 1;
ax + my ≡ 1 (mod m), 
removing the second term, we get  a.x ≡ 1 mod m which equals to the modular multiplicative inverse of an integer formula

calculating the MMI involves replacing the b as the new a and r as the new b and calculating the new remainder valuein "a = b.q + r" till you achieve r as 1 
Also recall that b = m in extended euclidean algorithm
