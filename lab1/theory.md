# Theory Questions

## 1, 2. See drawing on iPad


## 3. Why is recurrence correct?
One way to think of the recurrence is that it compares all the possible ways we can combine coins
to form $n$ value. The base case is, trivially, that we only use copper coins and thus end up with
$n$ coins. We are then comparing that to the amount of coins required to form $n - m$ value, plus
one, accounting for the coin we added, with $m$ here representing the value of a silver, gold or
platinum coin. To this value we are then adding one to account for the coin of said value we are
adding.


## 4. Variant
The new recurrence formula would be described by removing the option of using only copper coins,
meaning we'd remove the $n$ (which also can be interpreted as  $1 + \text{Coins}(n - 1)$) argument,
yielding the following. This would mean though that not every value of $n$ has a finite
$\text{Coins}(n)$ though. 

$$
\text{Coins}(n) = \begin{cases}
  \infty &\text{if } n < 0 \\
  0 &\text{if } n = 0 \\
  \min(
    1 + \text{Coins}(n - a),
    1 + \text{Coins}(n - b),
    1 + \text{Coins}(n - c)
  ) &\text{otherwise}
\end{cases}
$$

## 5. Description of f(x,y)
f(x,y) calculates the probability of having a win streak of y somewhere in all remaining x rounds.

1. If $y = 0$, then we're looking at the probability of achieving a win streak of 0, ie 100%.
2. If $x = 0$ and $y > 0$, there are no remaining rounds but we still want to find the probability
    of a non-zero a win streak. The probability of that is 0, there aren't enough rounds left.
3. Otherwise, the result is calculated through the probability of winning the current round
    (with probability $p$) multiplied by the probability of having a win streak of $y - 1$ in the
    $x - 1$ remaining rounds, plus the probability of losing the round multiplied by the
    probability of still having a new win streak of $k$ in the $x - 1$ remaining rounds.

Ie, (won game) * P(1 smaller win streak in 1 less rounds) + (lost game) * P(new win streak of in 1
less rounds)

The question is solved by $f(n, k)$, meaning the probability of having a win streak $k$ long in $n$
played rounds.


## 6, 7. Time Complexity
1a calls itself thrice for every call, which strongly suggests exponential growth. However, the
depth of the call stack is getting shorter by an average of about 6, (5+6+7)/3 = 6 meaning the
time complexity would be better described as $Θ(3^{n/6}) \sim Θ(1.2^n)$

In 1c when memoization is added, the number of calls per $n$ as $n$ grows large remains more or
less constant, meaning we can expect a linear time complexity: $Θ(n)$

Using an iterative approach in 1e, we are again performing $O(1)$ operations per $n$, thus
landing us on linear time complexity: $Θ(n)$

2a again has two recursive calls of $f$ per call of $f$, meaning we'd expect some exponential
growth. However when memoization is added, we're computing a finite number of subproblems per
increase of $n$. The time complexity thus drops to be independently linear in both arguments,
collapsing the time complexity from exponential to polynomial: $Θ(nk)$. We then set $k = n/2$,
which results in a final time complexity of $Θ(n^2)$.

2c has two calls of itself per call, but with memoization and the fact that the function only takes
a single variable input, the time complexity remains linear - a constant number of new computations
to do per increase of $n$.

