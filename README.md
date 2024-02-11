# Rust lambda function in AWS

In this repo two lambda Rust function were built.
The first one was constructed to have hands on Rust coding learning from basics. Thus the idea was to follow the Algorithms book by Cormen et al. and make and insertion-sort following the next pseudocode:


>1. For i from 2 to n
>2.   Key = A[i]
>3.   j = i - 1
>4.   While j > 0 and A[j] > Key
>5.     A[j+1] = A[j]
>6.     j = j - 1
>7.   End While
>8.   A[j+1] = Key
>9. End For


[Image]

In the second lambda function, the one which was deployed in AWS filter and count the amount of players that each Football clubs in MLS that earn more than ```N``` quantity.

[Image]


[Image]


