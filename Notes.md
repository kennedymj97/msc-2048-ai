# Notes

**Resources for benchmarking and profiling:**
[RustConf 2018 - Benchmarking and Optimization of Rust Libraries by Paul Mason](https://www.youtube.com/watch?v=d2ZQ9-4ZJmQ)
[RustConf 2017 - Improving Rust Performance Through Profiling and Benchmarking by Steve Jenson](https://www.youtube.com/watch?v=hTHp0gjWMLQ)

## Expectimax depth 3 timings

On PC (average over 20 runs):
Basic: 14.07secs
Optimised: 10.44secs

12/05/2020 - Improvements made to extracting columns and using hash maps for the shifting.
Optimised: 6.22secs

13/05/2020

1. Changed the move tables from hash maps to vectors ~90% performance boost.
2. Changed the score function to use a store, find score for every row and column and sum. ~75% performance boost.
    1. This has made the AI perform worse however so the times will not be representative, compared to old times, need to measure time per move instead.
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

## 02/07/2020

[x] Create function to generate all possible permutations of a given vector
[x] Function to test all of the permutations of a strategy
[x] Change snake AI to have fall-back sequence outside of rules

**Notes on ranking of strategies:**
It is not good enough to simply just sample a random number of times of my choice and then look at the median. Need to be more thorough in my analyses of the strategies.

Options:

1. Mann-Whitney U-test, this can compare 2 strategies and work out if there is a difference in performance. Useful resources: [Wikipedia](https://en.wikipedia.org/wiki/Mann%E2%80%93Whitney_U_test#cite_note-Pearce-1), [paper](https://reinventionjournal.org/article/view/339/416).
2. Shapiro-Wilk test. This is used to determine if the data has a normal distribution. If it does can then do calculations based on normal distribution to determine tests.
3. Other distribution tests. Might be worth exploring a bit more and seeing what the distributions normally are and then doing test for that/those.

## 03/07/2020

[x] Check if data is normally distributed
[x] Decide what method I will use to rank strategies
[] Start work on implementing the ranking code

**Notes:**

-   Tested a strategy and the data was not normally distributed.
-   Also tested for a lognormal distribution which it did not fit.
-   This paper is essential to the decision of test: [see here](https://reinventionjournal.org/article/view/339/416).
-   Can probably use either the Mann-Whitney U Test or the Yuen-Welch test, both seem to give good data. - Do whichever is easiest to implement first, if not suitable implement the other. - [Mann-Whitney U test info](https://www.lboro.ac.uk/media/wwwlboroacuk/content/mlsc/downloads/2.3_mann_whitney.pdf) - [Critical value tables (for if nx or ny is under 20)](https://www.real-statistics.com/statistics-tables/mann-whitney-table/) - Other useful resources: - [rustats](https://docs.rs/crate/rustats/0.1.0) - [statrs](https://docs.rs/statrs/0.7.0/statrs/distribution/trait.Univariate.html)

-   Open this file in excel copying the data and pasting it with transpose option (to convert from cols to rows)
-   Make sure I have the Shapiro-Wilk testing installed [(Real statistics resource pack)](https://www.real-statistics.com/free-download/real-statistics-resource-pack/).
-   Press ctrl+m whilst highlighting where you want the results to be output to.
-   Select the relevant cells, and check boxes and carry out tests

**Notes on implementing test:**

-   Use resources to help
-   Use excel to help produce test cases

## 06/07/2020

[x] Do draft 2 of project plan
[x] Decide on how to implement statistical performance analysis
[] Implement statistical performance analysis

**Project plan changes:**
[x] Section 1 Paragraph 1 - change goal of the project
[x] Section 1 Paragraph 2 - need to add some bit about previous student attempts
[x] Section 1 Paragraph 2 - add other ways the project may be interesting/important from a research perspective
[x] Section 2 Paragraph 1 - need to place project in context, talk about other games, chess and games where simple winning strategies exist
[x] Section 2 - Do I have the word count to add something about interpretability.
[x] Section 3.1 - Add a bit more detail (if word count allows) - Already over word count, leave out additional detail for now
[x] Section 3.2 - change description of how strategies will be compared for performance
[x] Section 4 - Edit developing strategies WP
[x] Section 4 - Add optional WP for writing journal paper?
[x] Section 5 - Update Gantt chart

**How to do statistical analysis of strategies:**

1. When running the strategies always save the data and conduct analysis separately. This will make the separation of tasks clearer and will also ensure a record of the raw data is kept.
2. Not sure of the exact details of how to do the comparison and keep best strategies so this may have to be adapted. At first use the following approach:
    1. Keep a list of the top strategies.
    2. When a new strategy is evaluated compare it to the first element. This will result in 3 scenarios:
        1. If the strategy is clearly worse simply move on.
        2. If there is no confidence of difference append to the top strategies.
        3. If the strategy is better, create a new list with the strategy as the first element. Compare every strategy in the previous top strategies to the new strategy using the same logic.
3. Hopefully this will always result in a small number of strategies that can then be further tested manually. If there are too many strategies a method of parsing them for further evaluation may need to be developed.

## 07/07/2020

[x] Fully implement statistical performance analysis
[] Do some research into better search methods to find strategies

## 08/07/2020

[x] Edit way strategy data is saved
[x] Create function to parse strategy data
[x] Implement function to find top strategies

**Thoughts on the next steps:**

-   Snake AI does seem to be running slowly - Profile to see if improvements possible, if so: - Set up benchmark - Profile and optimise
-   Expand language to include many more rules and brute force limiting max size - Automated method of generating all possible moves (maybe have a trait that will generate all possible variations of a rule? Or expand rule trait) - Test the work flow of finding best strategies - Produce some nice plots - Histograms for best strategies - Histograms of medians of all strategies
-   Once I have a robust set up for brute force and designing a language and testing it start using more advanced search methods

## 09/07/2020

[x] Work on optimising snake AI - All AIs now optimised, was previously creating the stores every time I ran the AI... Just need to make sure to create stores... Can change implementation if needed to force creation of stores.

## 10/07/2020

[x] Check create stores not being done more than once
[x] Refactor snake AI and related
[x] Work on generating all possible rule variations

**Next steps for brute force:**

-   Ensure the data is being saved in a format that can be loaded into excel
-   Implement a progressive brute force, gradually increase number of runs, saving a data file for every number of runs, only run strategies remaining based of Mann-Whitney - Check the progressive brute force is faster and produces equivalent results
-   Add more rules
-   Produce some nice plots

**Brute force vs progressive brute force:**
Time taken at max strategy size 4, 12 possible rules (13345 strategies), 1000 runs brute force: 25.54mins, usr time: 1509.18secs, sys time: 29.25secs (100 microseconds per game!)
Time taken at max strategy size 4, 12 possible rules (13345 strategies), 1000 runs progressive brute force: 33.72secs, usr time: 32.04secs, sys time: 1.67secs
Both the brute force and progressive brute force produced the same results.

## 11/07/2020

[x] Implement progressive brute force

**Progressive brute force:**

-   Top level function: - Create folder - Generate all strategies - Call aux function giving strategies and folder name
-   Aux function - Run all the strategies given storing the results in a data structure - Save the results to a file - Perform Mann-Whitney U test on results - Recursively call aux function to get results
-   When running strategies instead of just writing to a file, store the data in a vec
-   Recursively call the function until a certain number of strategies is reached

## 12/07/2020

[x] Time progressive brute force on pc
[x] Save files as csv

**Saving files as csv:**

-   Comma delimited, need to make the strategies print in a different way somehow

## 13/07/2020

[x] Refactor engine for safety, ensure stores are created
[x] Get detailed statistics on best current strategy
[] Add another rule

**Detailed statistics:**

-   run for 10000 games
-   data to record for each run: - score - highest tile reached

**Add another rule:**

-   Collect a 2 games that get 2048 and 2 games that do not, what is the key failure point, can an extra rule help?

**New rule ideas:**

-   How do I handle a right move?? If get bad placement can you do up move?
-   Add sequences (length 2) to force moves
-   Merge down when possible in left column

## 21/07/2020

[x] Work out which has more possibilities, including force move in main, or splitting
[x] Test the strategies with different fall-back rules
[x] Split snake up into 3 stages: Ban -> Try -> Force

## 22/07/2020

[x] Add new attributes: is column monotonic, is largest tile in corner
[x] Add the option of 99% confidence interval to Mann Whitney U test
[x] Ordering not important for ban rules, don't need to get permutations to get all possible orderings

## 23/07/2020

[x] Start looking into and planning dissertation
[x] Try using generics instead of trait objects functions that take an AI

**On implementing strategies without storing everything:**
Can an iterator be used to go through all the strategies.

## 27/07/2020

[x] Implement search for best strategies without storing everything, only store best strategies (this will prevent memory overload for very large search space)
[x] Start looking into and implementing iterative local search
[x] Work on writing dissertation

Memory used with 2 ban and 4 try currently: starts at 3400MB @ 7000000 at 10 runs, about 30% of memory or something
Memory used with 2 ban and 4 try after changing to iterator implementation is very small: only 0.1% of memory

**Resources for local search:**

-   [Local search algorithms](https://www.cs.unc.edu/~lazebnik/fall10/lec06_local_search.pdf)
-   [Iterated local search (Wikipedia)](https://en.wikipedia.org/wiki/Iterated_local_search)
-   [A good slide on what constitutes a good search technique](https://www.youtube.com/watch?v=Vye39FMb5vo)
-   [Introduces the idea of using a greedy algorithm as a starting point for local search](https://www.youtube.com/watch?v=XUNGtxoBbPQ)

**Steps to get to local search:**

1. Implement greedy algorithm
    - It will be interesting to see how well it performs
    - Can be used as a starting point for local search
2. Decide on local search method
    - How will a strategy be varied to try and reach the local maxima? 1. Try all possible moves for the first try move. 2. Try all possible moves for the ban moves (only used moves in try moves). 3. Try all possible moves for next try move. 4. Repeat step 2 -> 3 until all the try moves have been analysed.
3. Decide on a perturbation method
    - If wanting to do iterated local search need to find a way to try and jump out of the local maxima - vary the order of the try moves? - vary the composition of the try and ban moves?

## 28/07/2020

[x] Greedy search for strategies
[x] Make progressive brute force a bit quicker
[x] Measure progressive brute force time
[x] Don't allow repeated try moves in greedy
[x] Don't allow last try move to be same as first force move in greedy
[x] Only allow moves in try sequence in ban sequence in greedy
[x] Measure greedy time
[x] Enforce redundant snake not created in the constructor

**Greedy search:**

1. Find best try move at position 1
2. Continue finding best try move until a max length has been reached
3. Repeat for ban moves until max ban length has been reached

**Progressive brute force time:** 
- Before changing rule generation and not allowing redundant snakes: ~11800000 snakes @ 2 ban 4 try varying fall-back, 62 mins -> 5, 50, 500, 5000, 50000 (number of runs of each set)
- After changes: ~180000 snakes @ 2 ban 5 try fixed fall-back, 337secs -> 5, 50, 500, 5000, 50000 (number of runs of each set)
**Greedy time:** 107secs

There is some bug in the progressive brute force code where the same strategy is repeated many times. Need to work out what it is and fix it. Execute on smaller number of possibilities to test it.

## 29/07/2020

[x] Fix bug where same strategy is being used multiple times
[x] Remeasure progressive brute force time with new try rule generation
[x] Brute force a good number of strategies for 100 runs so I can plot a good histogram
[x] Set up python to handle and plot the data
[x] Produce plot of the medians
[x] Produce histogram plot

- As the number of rules increases the number of good strategies increases greatly, the early rules have much more effect than the later rules.
- Before forcing moves, should I go through the same ordering but instead trying to make the moves
	- This means banned moves would have some effect after the try sequence

## 30/07/2020

[x] Try changing the fall-back rules so they will try to make the moves before forcing, ban rules will have more effect
[x] Adapt the greedy search
[x] Adapt Mann-Whitney to be more adaptable, either P05 or P01
[x] Clean up greedy search code, allow picking best, prioritising try and prioritising ban, choose the confidence level and max runs until just choosing current best
[x] Add always rules

Changing the fall-back rules so they will try first has had a positive impact on performance.

**New top snake:**
Getting stats for best strategy...
Strategy: Ban Rules: ban move Up if left column locked->ban move Right if left column locked->ban move Left if breaks monotonicity of left column       Try Rules: try move Left if moves largest tile to corner->try move Down if moves largest tile to corner->try move Left if merge possible->try move Up if produces left merge->try move Down if produces left merge->try move Up if merge possible        Fall-back: Left->Up->Down->Right
Median: 11708
Average: 12021.85084

## 31/07/2020

[x] Make the current rules more general
[x] Add rules for 2 largest tiles
[x] Change try move if produces merge to work along rows/columns
[x] Remove unnecessary comparisons causing early exit from greedy
[x] Fix right at back of fall-back variations (okay because of symmetry)
[x] Edit try if produces merge rule so it doesn't need to consider direction
[x] Row monotonic attribute
[x] Ban move if breaks monotonicity of row
[x] Try move if makes row/column monotonic

## 04/08/2020

[x] Try move if locks row/column
[x] Ban move if unlocks row/column
[x] Ban move if removes potential merge
[x] Ban move if moves largest tile out of corner
[x] Remove unneeded attribute produces merge in direction
[x] Try move if row/column locked

## 05/08/2020

[x] Update the graphs with what was discussed in meeting last week
[x] Calculate size of search space
[x] Try move if empties column/row
[x] Ban move if fills column/row

## 09/08/2020
[x] Refactor search code
[x] Add random strategy generator

## 10/08/2020
[x] Test random strategy generation
[x] Add random search for strategy
[x] Add max ban and try length to greedy searches
[x] Local search for strategies (on top of greedy and random)
[] Add in some permutation for iterated local search
[~] Add filter for greedy search (not sure if I want to complete this now, instead just set a limit on max ban/try??)

**Local search thoughts:**
- There are 2 possible alternatives:
	1. Every time a rule is changed restart the search from the beginning
	2. Go through every strategy finding the best strategy, restart at the end if any strategy has been changed
- Exit if there is no change from the original strategy

**Iterated local search thoughts:**
[Good info on iterated local search](https://www.eit.lth.se/fileadmin/eit/courses/ets061/Lectures/Lecture-H2.pdf)
- The are 2 options after finding local optimum after perturbation
	1. Only accept the new local optimum if it is better than old
	2. Always accept the new local optimum
- Exit after a certain number of iterations
Have made good progress on iterated local search, but I think there are some bugs. The local search does not seem to be changing rules after they are randomly changed. Sometimes more than one rule is changed randomly. Not sure if this is and issue with the permutation code or the swapping code. Maybe implement some more tests.

## Unallocated
[] Fix evaluation of strategies from file
[] Write formal description of language

