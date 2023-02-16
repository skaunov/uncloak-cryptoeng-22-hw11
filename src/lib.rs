use group::Group;
use integer_sqrt::IntegerSquareRoot;
// integer-sqrt can be easily ditched

// `usize` is an arbitrary choice, also see https://users.rust-lang.org/t/integer-square-root/96/6
// pub fn shank_steps<GroupEl>(element: GroupEl, order: usize, operation: fn(GroupEl, GroupEl) -> GroupEl) 
pub fn shank_steps<
    GroupEl: 
        std::clone::Clone
        + std::marker::Copy
        + std::cmp::PartialEq
>(
    element: GroupEl, 
    order: usize, 
    value_pow: GroupEl, 
    operation: fn(GroupEl, GroupEl) -> GroupEl,
    inverse_fn: fn(GroupEl) -> GroupEl
) -> usize
{
    let l = order.integer_sqrt() + 1;
    // let l = if l * l < order {l + 1} else {l};impl
    println!("DEBUG! `l` {:?}", l);
    
    // ~~it's not needed, just use modpow as Mul~~
    // potential oprimisation is borrowing instead of cloning
    let power = |base: GroupEl, exp: usize| -> GroupEl {
        let mut counter = exp;
        let mut result = base.clone();
        while counter > 1 {
        // for _ in 0..=exp {
            counter -= 1;
            result = operation(result, base.clone());
        }
        result
    };

    let mut steps_baby = Vec::new();
    let mut steps_giant = Vec::new();

    steps_baby.push(operation(element.clone(), inverse_fn(element.clone())));
    let mut runner = element.clone();
    steps_baby.push(runner);
    for i in 1..=l {
        runner = operation(runner, element.clone());
        steps_baby.push(runner);
    }

    /* it isn't really documented AFAICS what `Neg` trait in the context of `group` crate means
    as I can't find any concept of inverse in `group`, let me propose that `Neg` should define inverse
        of the group operation, since inverse must be defined for the structure to be a group */
    let multiplicator = power(inverse_fn(element), l);
    println!("DEBUG! `multiplicator` {:?}", multiplicator);
    let mut runner = value_pow;
    steps_giant.push(runner);
    for i in 1..=l {
        runner = operation(runner, multiplicator.clone());
        steps_giant.push(runner);
    }

    let mut i = usize::default();
    let mut j = usize::default();

    println!("DEBUG! babies {:?}", steps_baby);
    println!("DEBUG! giants {:?}", steps_giant);

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
    println!("DEBUG! `i`ndex {}", i);
    println!("DEBUG! `j` index {}", j);
    i + j * l

}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn modinv() {
        assert_eq!(modinverse::modinverse(9704usize, 17389).unwrap(), 14943);
    }

    #[test]
    fn modinvb() {
        assert_eq!(bubblemath::number_theory::mod_inv(9704, 17389), 14943);
    }
    
    /* let's try to fill `Group` trait
    with just what <crates.io> offers */
    #[test]
    fn collision() {
        // 9704^x ≡ 13896 (mod 17389)
        const modulo: usize = 17389;

        use num::bigint::*;
        use num_modular::ModularCoreOps;
        fn binded_op(f: usize, s: usize) -> usize {
            f.mulm(&s, &17389usize)
        }
        // fn binded_inv(left: usize) -> usize {
        //     println!("{left}");
        //     modinverse::modinverse(left, modulo)
        //         .unwrap()
        // }
        fn binded_inv(left: usize) -> usize {
            println!("DEBUG! {}", left as u64);
            let r = bubblemath::number_theory::mod_inv(left as u64, modulo as u64);
            println!("DEBUG! {}", r);
            r as usize
        }
        let x = shank_steps(
            9704usize, 
            1242, 
            13896usize, 
            binded_op, 
            binded_inv
        );
        let value_assert = modpow::modpow(
            &9704.to_bigint().unwrap(), &x.to_bigint().unwrap(), &modulo.to_bigint().unwrap()
        );
        println!("DEBUG! {}", value_assert);
        assert_eq!(
            value_assert, 
            13896.into()
        );
    }
}