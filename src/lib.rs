/// An exercise implementation of Shanks Baby-step giant-step for arbitrary groups. No optimizations were kept in mind for it. The idea is to create a function which would work with any group, so
/// it needs to be provided needed information about the group itself (`operation` and inverse) and the `order` of the element that was exponentiated.

use num::integer::Roots;

// `usize` is an arbitrary choice, also see https://users.rust-lang.org/t/integer-square-root/96/6
/// Takes 
/// * an `element` of the group which would act as the "base of logarithm",
///   * its `order`, square root of which defines size of the problem;
/// * `value_pow` as the "argument to logarithm";
/// * group's binary `operation`;
/// * and unary `inverse_fn` which returns inverse element in the group.
pub fn shank_steps<
    GroupEl: 
        std::clone::Clone
        + std::cmp::PartialEq
>(
    element: GroupEl, 
    order: usize, 
    value_pow: GroupEl, 
    operation: fn(GroupEl, GroupEl) -> GroupEl,
    inverse_fn: fn(GroupEl) -> GroupEl
) -> usize
{
    let l = order.sqrt() as usize + 1;
    let identity = operation(element.clone(), inverse_fn(element.clone()));
    
    // maybe it would be better to borrow instead of cloning
    let power = |base: GroupEl, exp: usize| -> GroupEl {
        let mut result = identity.clone();
        let mut exp = exp;
        let mut base = base;
        while exp > 0 {
            if exp % 2 == 1 {result = operation(result, base.clone());}             
            exp >>= 1;
            base = operation(base.clone(), base);
        }
        result
    };

    let mut steps_baby = Vec::new();
    let mut steps_giant = Vec::new();

    // here's a small catch-up: Shanks baby-step list starts with identity element
    steps_baby.push(identity.clone());

    let mut runner = element.clone();
    steps_baby.push(runner.clone());
    for _ in 1..=l {
        runner = operation(runner.clone(), element.clone());
        steps_baby.push(runner.clone());
    }

    let multiplicator = power(inverse_fn(element), l);
    let mut runner = value_pow;
    steps_giant.push(runner.clone());
    for _ in 1..=l {
        runner = operation(runner.clone(), multiplicator.clone());
        steps_giant.push(runner.clone());
    }

    let mut i = usize::default();
    let mut j = usize::default();

    // TODO check that answer is actually single
    steps_baby.iter().enumerate()
        .for_each(|(pos_baby, el_baby)| {steps_giant.iter().enumerate().for_each(
            |(pos_giant, el_giant)| {
                if el_baby == el_giant {
                    i = pos_baby;
                    j = pos_giant;
                }
            }
    )});
    i + j * l

}

#[cfg(test)]
mod tests {
    use super::*;

    // Couple of test used for debugging.
    #[ignore]
    #[test]
    fn modinv() {
        assert_eq!(modinverse::modinverse(9704usize, 17389).unwrap(), 14943);
    }

        // Second one of the couple of test used for debugging.
    #[test]
    fn modinvb() {
        assert_eq!(bubblemath::number_theory::mod_inv(9704, 17389), 14943);
    }
    
    /* The idea was just to drop-in everything suitable from <Crates.io> to see that higher-order function could deal with it. It's quite simplistic with the integer group, could worth to try
    with EC group, or other. */
    #[test]
    fn collision() {
        // 9704^x ≡ 13896 (mod 17389) // testing vector is taken from the book
        const MODULO: usize = 17389;

        use num::bigint::*;
        use num_modular::ModularCoreOps;
        fn binded_operation(f: usize, s: usize) -> usize {
            f.mulm(&s, &17389usize)
        }
        
            // During debugging different versions of mod. inv. were tried.
        // fn binded_inv(left: usize) -> usize {
        //     println!("{left}");
        //     modinverse::modinverse(left, modulo)
        //         .unwrap()
        // }
        fn binded_inv(left: usize) -> usize {
            println!("DEBUG! {}", left as u64);
            let r = bubblemath::number_theory::mod_inv(left as u64, MODULO as u64);
            println!("DEBUG! {}", r);
            r as usize
        }
        
        let x = shank_steps(
            9704usize, 
            1242, 
            13896usize, 
            binded_operation, 
            binded_inv
        );

        // Asserting on using result as exponent looks more general than comparing the resulting value of "logarithm".
        let value_assert = modpow::modpow(
            &9704.to_bigint().unwrap(), &x.to_bigint().unwrap(), &MODULO.to_bigint().unwrap()
        );
        assert_eq!(
            value_assert, 
            13896.into()
        );
    }
}