# Week 11 homework for Uncloak CryptoEng course

Exercise implementation of Shanks Baby-step giant-step for arbitrary groups. No optimizations were kept in mind for it. The idea 
is to create a function which would work with any group, so
it needs to be provided with information about the group itself (`operation` and inverse) and the `order` of the element 
that was exponentiated.

## Review
https://discord.com/channels/1031896857074475059/1031917142578311229/1075861221993087146
> A couple quick notes: your algorithm to compute exponents is linear in the size of the exponent; it could be logarithmic by using the fast-powering algorithm. 
You could be using the Fn in place of fn; fn could be FnMut, FnOnce, or Fn, or even just implement shank step as a method over a Group struct, with defined methods for operation and inverse, though your impl is nice self-contained for a proof of concept.
You could compute both baby-steps and giant-steps in the same loop; you could store the values for index (Value -> Index, not the other way around) and value in a pair of hashmaps, and check at each addition whether the value already existed in the other hashmap, and terminate early if found. Finally, cargo clippy would find some generic lints.