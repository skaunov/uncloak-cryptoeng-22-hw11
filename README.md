# Week 11 homework for Uncloak CryptoEng course

Exercise implementation of Shanks Baby-step giant-step for arbitrary groups. No optimizations were kept in mind for it. The idea 
is to create a function which would work with any group, so
it needs to be provided with information about the group itself (`operation` and inverse) and the `order` of the element 
that was exponentiated.
