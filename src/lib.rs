use group::Group;
use integer_sqrt::IntegerSquareRoot;
// integer-sqrt can be easily ditched

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


// `usize` is arbitrary choice, also see https://users.rust-lang.org/t/integer-square-root/96/6
// pub fn shank_steps<GroupEl>(element: GroupEl, order: usize, operation: fn(GroupEl, GroupEl) -> GroupEl) 
pub fn shank_steps<GroupEl: Group>(
    element: GroupEl, order: usize, value_pow: GroupEl
) -> usize
{
    let l = order.integer_sqrt() + 1;
    // let l = if l * l < order {l + 1} else {l};impl
    
    // ~~it's not needed, just use modpow as Mul~~
    // potential oprimisation is borrowing instead of cloning
    let power = |base: GroupEl, exp: usize| -> GroupEl {
        let mut exp = exp;
        let mut result = base.clone();
        while exp > 0 {
            exp -= 1;
            result = result + base.clone();
        }
        result
    };

    let mut steps_baby = Vec::new();
    let mut steps_giant = Vec::new();

    let mut runner = element.clone();
    steps_baby.push(runner);
    for i in 1..=l {
        runner = runner + element.clone();
        steps_baby.push(runner);
    }

    /* it isn't really documented AFAICS what `Neg` trait in the context of `group` crate means
    as I can't find any concept of inverse in `group`, let me propose that `Neg` should define inverse
        of the group operation, since inverse must be defined for the structure to be a group */
    let multiplicator = power(-element, l);
    let mut runner = value_pow;
    steps_giant.push(runner);
    for i in 1..=l {
        runner = runner + multiplicator.clone();
        steps_giant.push(runner);
    }

    let mut i = usize::default();
    let mut j = usize::default();
    // TODO check that answer is actually single
    steps_baby.iter().enumerate().for_each(|(pos_baby, el_baby)| {steps_giant.iter().enumerate().for_each(
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

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    /* let's try to fill `Group` trait
    with just what <crates.io> offers */
    struct TestGr {}
    impl Group for TestGr {
        
    }
}


// funny snippet from dumb path I took; I like the list
// pub fn shank_steps/* <
//     Group: 
//         std::ops::Add<Output = Group> 
//         + std::ops::Neg<Output = Group> 
//         + std::clone::Clone
//         + std::marker::Copy
//         + std::cmp::PartialEq
// > */(