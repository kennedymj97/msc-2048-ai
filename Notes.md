# Notes

## Useful Benchmarking Resources
[RustConf 2018 - Benchmarking and Optimization of Rust Libraries by Paul Mason](https://www.youtube.com/watch?v=d2ZQ9-4ZJmQ)  
[RustConf 2017 - Improving Rust Performance Through Profiling and Benchmarking by Steve Jenson](https://www.youtube.com/watch?v=hTHp0gjWMLQ)

## Expectimax depth 3 timings

On PC (average over 20 runs):  
Basic: 14.07s  
Optimised: 10.44s  
  
12/05/2020 - Improvements made to extracting columns and using hashmaps for the shifting.  
Optimised: 6.22s  
  
13/05/2020  
1. Changed the move tables from hashmaps to vectors ~90% performance boost.
2. Changed the score function to use a store, find score for every row and column and sum. ~75% performance boost.
    1. This has made the ai perform worse however so the times will not be representative, compared to old times, need to measure time per move instead.
    1. Current average move_time 3-6ms.
3. Refactor to remove the basic engine, started work on optimising the evaluation of chance nodes.

14/05/2020  
1. Improved evaluate chance function ~35% improvement.
    1. Average move time down to 0.5-2ms (at depth 3).
2. Changed evaluate chance so no loop is used, meaning no vector alloc. ~32% improvement.
    1. Average move time down to 0.5-1.5ms (at depth 3).
3. Using a static store, requires unsafe code, but makes the copying of the structs more efficient.
    1. Average move time down to 0.4-1ms (at depth 3).

15/05/2020
1. Added a transposition table.
    1. Average move time down to 0.1-0.2ms (at depth 3).


## Platform performance
WSL2 - 17.3 moves/sec at max depth 6
Firefox (windows) - 14.4 moves/sec at max depth 6

# Day Notes
### 02/07/2020
TODO:
[x] Create function to generate all possible permutations of a given vector
[x] Function to test all of the permutations of a strategy
[x] Change snake AI to have fallback sequnce outside of rules

**Notes on ranking of strategies:**
It is not good enough to simply just sample a random number of times of my choice and then look at the median. Need to be more thorough in my analyses of the strategies.

Options:
1. Mann-Whitney U-test, this can compare 2 strategies and work out if there is a difference in performance. Useful resources: [wikipedia](https://en.wikipedia.org/wiki/Mann%E2%80%93Whitney_U_test#cite_note-Pearce-1), [paper](https://reinventionjournal.org/article/view/339/416).
2. Shapiro-Wilk test. This is used to determine if the data has a normal distribution. If it does can then do calculations based on normal distribution to determine tests.
3. Other distribution tests. Might be worth exploring a bit more and seeing what the distributions normally are and then doing test for that/those.

### 03/07/2020

