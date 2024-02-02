# Lab 1: Dynamic Programming

## Task 1: Coin Change

We first look at a basic coin changing problem which can be solved using dynamic programming. The
problem is described in [kth.algokomp.coinchange](https://kth.kattis.com/courses/DD2352/algokomp24/assignments/hnrhei/problems/kth.algokomp.coinchange).
Read through the problem description there and make sure you understand what the problem is asking
for.

A recurrence that describes the solution to the problem is the following:
$$
\text{Coins}(n) = \begin{cases}
   \infty &\text{if } n < 0 \\
   0 &\text{if } n = 0 \\
   \min(
      n,
      1 + \text{Coins}(n - a),
      1 + \text{Coins}(n - b),
      1 + \text{Coins}(n - c)
    ) &\text{otherwise}
\end{cases}
$$

**(a)** Write a direct recursive implementation (without any dynamic programming component) of this
recurrence.

Submit this implementation to the Kattis problem. It will likely get a "Time Limit Exceeded
Result". *In order to pass this part of the lab, your implementation here must pass at least the
first 33 test cases on Kattis.*

**(b)** Fix the values a = 5, b = 6, c = 7 (the same ones used in the second sample input on
Kattis) and time your code for increasing values of n. How large instances (what value of n) can
your program solve in 1 second (on your machine)? If you take that n and increment it by one at a
time (i.e., try n+1, n+2, n+3 etc), how does the runtime change? If you instead take that n and
then repeatedly double it (i.e. try 2\*n, 4\*n, 8\*n, etc), how does the runtime change? Tabulate
and/or plot the results. Based on these experiments, what would be a reasonable estimate of the
time complexity of your program?

**(c)** Make a new implementation, based on the one from (a), but where you add memoization to save
computed values of the Coins() function and reuse them when they are needed again, thereby turning
this into a dynamic programming solution.

Submit this implementation to the Kattis problem. *In order to pass this part of the lab, your
implementation here must get an Accepted result on Kattis.*

**(d)** Repeat subtask (b) but on the dynamic programming solution from (c):
Fix the values a = 5, b = 6, c = 7 (the same ones used in the second sample input on Kattis) and
time your code for increasing values of n. How large instances (what value of n) can your program
solve in 1 second (on your machine)? If you take that n and increment it by one at a time (i.e.,
try n+1, n+2, n+3 etc), how does the runtime change? If you instead take that n and then repeatedly
double it (i.e. try 2\*n, 4\*n, 8\*n, etc), how does the runtime change? Tabulate and/or plot the
results. Based on these experiments, what would be a reasonable estimate of the time complexity of
your program?

**(e)** Make a bottom-up implementation of this algorithm. In other words, instead of a recursive
algorithm, write an iterative algorithm with a for loop that tabulates the values of Coins() from
small to large. *In order to pass this part of the lab, your implementation here must get an
Accepted result on Kattis.*

Compare the running time of this program with the one from (c). What differences are there, if any?

## Task 2: Winning Streaks

We now look at another problem which can be solved with dynamic programming in a few different
ways. The problem we will solve is
[kth.algokomp.winningstreak](https://kth.kattis.com/courses/DD2352/algokomp24/assignments/hnrhei/problems/kth.algokomp.winningstreak).
Read through the problem description there and make sure you understand what the problem is asking
for.

**(a)** A basic recurrence that can be used to compute the answer is to define, for integers
$0 \leq x \leq n$ and $0 \leq y \leq k$,
$$
f(x,y) = \begin{cases}
  1.0 &\text{if } y = 0 \\
  0.0 &\text{if } x = 0 \text{ and } y > 0 \\
  p \cdot f(x - 1, y - 1) + (1 - p) \cdot f(x - 1, k)
    &\text{otherwise (if } x \geq 1 \text{ and } y \geq 1 \text{)}
\end{cases}
$$
The answer is then given by f(n, k).  Note that the second term in the above recurrence has k as
the second argument, and not y (so the value of y is "reset" to k).

Implement this recurrence as a recursive algorithm and add memoization to turn it into a dynamic
programming solution.

Submit your solution Kattis. It will likely get a "Time Limit Exceeded Result". *In order to pass
this part of the lab, your implementation here must pass the first 22 test cases on Kattis.*

**(b)** Test your solution with different values of n, and use the values k = n/2 and p = 0.99. How
large instances (what value of n) can your program solve in 1 second (on your machine)? If you take
that n and increment it by one at a time (i.e., try n+1, n+2, n+3 etc), how does the runtime
change? If you instead take that n and then repeatedly double it (i.e. try 2\*n, 4\*n, 8\*n, etc),
how does the runtime change? Tabulate and/or plot the results. Based on these experiments, what
would be a reasonable estimate of the time complexity of your program?

**(c)** Another recurrence that can be used to solve the problem is to define, for an integer
$0 \leq x \leq n$,
$$
g(x) = \begin{cases}
  0.0 &\text{if } x < k \\
  p^k &\text{if } x = k \\
  g(x - 1) + p^k \cdot (1 - p) \cdot (1 - g(x - k - 1))
    &\text{otherwise (if } x \geq k + 1 \text{)}
\end{cases}
$$
The answer is then given by g(n).

Implement this recurrence as a recursive algorithm and add memoization to turn it into a dynamic
programming solution.

Submit your solution to Kattis. *In order to pass this part of the lab, your solution must get the
result "Accepted" on Kattis.*


**(d)** Repeat task (b) but on the new solution from (c):
Test your solution with different values of n, and use the values k = n/2 and p = 0.99. How large
instances (what value of n) can your program solve in 1 second (on your machine)? If you take that
n and increment it by one at a time (i.e., try n+1, n+2, n+3 etc), how does the runtime change? If
you instead take that n and then repeatedly double it (i.e. try 2\*n, 4\*n, 8\*n, etc), how does
the runtime change? Tabulate and/or plot the results. Based on these experiments, what would be a
reasonable estimate of the time complexity of your program?

## Theory Questions
1. Simulate what the solution to 1(a) will do on the input n=15, a=5, b=6, c=7: draw the tree of
recursive calls that the recurrence gives rise to.
2. How will the recursive calls made in the memoized version in 1(c) differ from in this case
(n=15, a=5, b=6, c=7)?
3. Study the recurrence Coins(n) in problem 1. Why is this a correct recurrence that solves the
problem?
4. Suppose we were trying to solve a variant of the coin change problem where we only had silver,
gold and platinum coins at our disposal, no copper coins.  How would the recurrence describing the
answer to the problem change in this variant?
5. Study the function f(x, y) defined in subproblem 2(a). What does the value of f(x, y) represent,
in words? Argue why computing f(n, k) gives a solution to the problem.
6. Of the five different programs that you will write (in problems 1(a), 1(c), 1(e), 2(a), and
2(c)), which ones will have exponential time complexity, and which ones will have polynomial time
complexity?
7. Of the ones with polynomial time complexity, which ones will have linear time complexity?

8. *__Bonus question__. You do not need to answer this question to obtain the theory bonus point,
but if you want to understand why the two different recurrences in Task 2 yield the same answer,
this question guides you to that.*
Consider the two functions f(x, y) and g(x) defined in subproblems 2(a) and 2(b). Prove using
induction on x that f(x, k) = g(x) for all x. This can be a little challenging, so here are hints
about how to proceed:
    - It can be helpful to define an intermediate function h(x), which has the same base cases as
    g(x) but in the inductive case (when x >= k+1) is defined by
    $h(x) = p^k + \sum_{i=1}^{k-1} (1 - p) \cdot p^i \cdot h(x - i - 1)$
    - Prove that h(x) = f(x, k) for all x using induction.  This is conceptually not difficult but
    involves somewhat tedious calculations: we take the recursive definition of f(x, k) and expand
    it, getting terms involving f(x-1, k-1) and f(x-1, k).  For the terms of the form f(x-1, k) we
    use the inductive hypothesis to identify them with h(x-1), and for the terms of the form
    f(x-1, k-1) we again expand using the recursive definition getting terms involving f(x-2, k-2)
    and f(x-2, k).  We continue in the same way: whenever we get terms of the form f(x', k) we
    replace them by h(x'), and then we will also have terms of the form f(x-i, k-i) which we keep
    expanding using the recursive definition until we get f(x-k, 0) which is a base case and can
    replaced by a constant.
    - After this the remaining thing to prove is that h(x) also equals g(x). Here there is a nice
    "trick" that can make things work out in a nice way, which is to look at the quantity
    $h(x) - p \cdot h(x - 1)$, expand both terms using the recurrence for h(x), and simplify the
    resulting expression. The resulting identity should then lead towards the conclusion that in
    fact h(x)=g(x).