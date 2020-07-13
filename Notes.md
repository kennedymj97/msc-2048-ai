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
**TODO:**
[x] Check if data is normally distributed
[x] Decide what method I will use to rank strategies
[] Start work on implementing the ranking code

**Notes:**
- Tested a strategy and the data was not normally distributed.
- Also tested for a lognormal distribution which it did not fit.
- This paper is essential to the decision of test: [see here](https://reinventionjournal.org/article/view/339/416).
- Can probably use either the Mann-Whitney U Test or the Yuen-Welch test, both seem to give good data.
	- Do whichever is easiest to implement first, if not suitable implement the other.
	- [Mann-Whitney U test info](https://www.lboro.ac.uk/media/wwwlboroacuk/content/mlsc/downloads/2.3_mann_whitney.pdf)
	- [Critical value tables (for if nx or ny is under 20)](https://www.real-statistics.com/statistics-tables/mann-whitney-table/)
	- Other useful resources:
		- [rustats](https://docs.rs/crate/rustats/0.1.0)
		- [statrs](https://docs.rs/statrs/0.7.0/statrs/distribution/trait.Univariate.html)

**How to check if data is normally distibuted:**
- Run the strategy saving the scores in a comma delimited text file.
- Open this file in excel copying the data and pasting it with transpose option (to convert from cols to rows)
- Make sure I have the shapiro-wilk testing installed [(Real statistics resource pack)](https://www.real-statistics.com/free-download/real-statistics-resource-pack/).
- Press ctrl+m whilst highlighting where you want the results to be output to.
- Select the relevant cells, and checkboxes and carry out tests

**Notes on implementing test:**
- Use resources to help
- Use excel to help produce test cases

### 06/07/2020
**TODO:**
[x] Do draft 2 of project plan
[x] Decide on how to implement statistical performance analysis
[] Implement statistical performance analysis

**Project plan changes:**
[x] Sec1 Para1 - change goal of the project
[x] Sec1 Para2 - need to add some bit about previous student attempts
[x] Sec1 Para2 - add other ways the project may be interesting/important from a research perspective
[x] Sec2 Para1 - need to place project in context, talk about other games, chess and games where simple winning strategies exist
[x] Sec2 - Do I have the word count to add something about interpretability.
[x] Sec3.1 - Add a bit more detail (if word count allows)
	- Already over word count, leave out additional detail for now
[x] Sec3.2 - change description of how strategies will be compared for performance
[x] Sec4 - Edit developing strategies WP
[x] Sec4 - Add optional WP for writing journal paper?
[x] Sec5 - Update gantt chart

**How to do statistical analysis of strategies:**
1. When running the strategies always save the data and conduct analysis seperately. This will make the seperation of tasks clearer and will also ensure a record of the raw data is kept.
2. Not sure of the exact details of how to do the comparison and keep best strategies so this may have to be adapted. At first use the following approach:
	1. Keep a list of the top strategies.
	2. When a new strategy is evaluated compare it to the first element. This will result in 3 scenarios:
		1. If the strategy is clearly worse simply move on.
		2. If there is no confidence of difference append to the top strategies.
		3. If the strategy is better, create a new list with the strategy as the first element. Compare every strategy in the previous top strategies to the new strategy using the same logic.
3. Hopefully this will always result in a small number of strategies that can then be further tested manually. If there are too many strategies a method of parsing them for further evaluation may need to be developed.

### 07/07/2020
**TODO:**
[x] Fully implement statistical performance analysis
[] Do some research into better search methods to find strategies

### 08/07/2020
**TODO:**
[x] Edit way strategy data is saved
[x] Create function to parse strategy data
[x] Implement function to find top strategies

**Thoughts on the next steps:**
- Snake AI does seem to be running slowly
	- Profile to see if improvements possible, if so:
		- Set up benchmark
		- Profile and optimise
- Expand language to include many more rules and brute force limiting max size
	- Automated method of generating all possible moves (maybe have a trait that will generate all possible variations of a rule? Or expand rule trait)
	- Test the workflow of finding best strategies
	- Produce some nice plots
	- Histograms for best strategies
	- Histograms of medians of all strategies
- Once I have a robust set up for brute force and designing a language and testing it start using more advanced search methods

### 09/07/2020
**TODO:**
[x] Work on optimising snake AI
	- All AIs now optimised, was previously creating the stores every time I ran the AI... Just need to make sure to create stores... Can change implementation if needed to force creation of stores.

### 10/07/2020
**TODO:**
[x] Check create stores not being done more than once
[x] Refactor snake AI and related
[x] Work on generating all possible rule variations

**Next steps for brute force:**
- Ensure the data is being saved in a format that can be loaded into excel
- Implement a progressive brute force, gradually increase number of runs, saving a data file for every number of runs, only run strategies remaining based of Mann-Whtiney
	- Check the progressive brute force is faster and produces equivalent results
- Add more rules
- Produce some nice plots

**Brute force vs progressive brute force:**
Time taken at max strategy size 4, 12 possible rules (13345 strategies), 1000 runs brute force: 25.54mins, usr time: 1509.18s, sys time: 29.25s (100 microseconds per game!)
Time taken at max strategy size 4, 12 possible rules (13345 strategies), 1000 runs progressive brute force: 33.72s, usr time: 32.04s, sys time: 1.67s
Both the brute force and progressive brute force produced the same results.

### 11/07/2020
**TODO:**
[x] Implement progressive brute force

**Progressive brute force:**
- Top level function:
	- Create folder
	- Generate all strategies
	- Call aux function giving strategies and folder name
- Aux function
	- Run all the strategies given storing the results in a data structure
	- Save the results to a file
	- Perform Mann-Whitney U test on results
	- Recursively call aux function to get results
- When running strategies instead of just writing to a file, store the data in a vec
- Recursively call the function until a certain number of strategies is reached

### 12/07/2020
**TODO:**
[x] Time progressive brute force on pc
[x] Save files as csvs

**Saving files as csvs:**
- Comma delimited, need to make the strategies print in a different way somehow

### 13/07/2020
**TODO:**
[x] Refactor engine for safety, ensure stores are created
[x] Get detailed statistics on best current strategy
[] Add another rule

**Detailed statistics:**
- run for 10000 games
- data to record for each run:
	- score
	- highest tile reached

**Add another rule:**
- Collect a 2 games that get 2048 and 2 games that do not, what is the key failure point, can an extra rule help?

**New rule ideas:**
- How do I handle a right move?? If get bad placement can you do up move?
- Add sequences (length 2) to force moves
- Merge down when possible in left column

### 14/07/2020
**TODO:**
[] Work out which has more possibilities, including force move in main, or splitting
[] Test the strategies with different fallback rules
[] Split snake up into 3 stages: Ban -> Try -> Force?


