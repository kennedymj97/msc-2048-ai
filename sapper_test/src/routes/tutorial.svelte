<script>
  import Rule from '../components/rule.svelte';

  let rules = [{name: "Ban move up if left column not locked", description: "If the leftmost column is full and no merges are possible it is 'locked'. Making a move in the vertical direction will not change the state fo the column. For this rule, if the leftmost column is NOT locked the the move up is banned", examples: [{imagePath: "./test.jpg", result: "Ban move up", reason: "The leftmost column is not full. A move in the vertical direction (up or down) will shift the tiles."}, {imagePath: "./test.jpg", result: "No ban", reason: "The leftmost column is full of tiles and no merge is possible in the vertical direction. The tiles will not shift if a move is made in the vertical direction"}, {imagePath: "./test.jpg", result: "Ban move up", reason: "The left column is full of tiles but a merge is possible. A move in the vertical direction will cause the merge to happen and the tiles will shift."}]},
{name: "Ban move up if left column not locked", description: "If the leftmost column is full and no merges are possible it is 'locked'. Making a move in the vertical direction will not change the state fo the column. For this rule, if the leftmost column is NOT locked the the move up is banned", examples: [{imagePath: "./test.jpg", result: "Ban move up", reason: "The leftmost column is not full. A move in the vertical direction (up or down) will shift the tiles."}, {imagePath: "./test.jpg", result: "No ban", reason: "The leftmost column is full of tiles and no merge is possible in the vertical direction. The tiles will not shift if a move is made in the vertical direction"}, {imagePath: "./test.jpg", result: "Ban move up", reason: "The left column is full of tiles but a merge is possible. A move in the vertical direction will cause the merge to happen and the tiles will shift."}]}];
</script>

<svelte:head>
  <title>Introduction</title>
</svelte:head>

<h1 class="mt-4 mb-1 text-3xl font-semibold">Tutorial</h1>
This tutorial will describe how to follow the strategy. Please read it before
doing the testing. You can come back to this page at any time during the
testing, your progress in the game will be saved.

<h2 class="mt-4 mb-1 text-2xl font-semibold">What is a strategy?</h2>
A strategy consists of a set of rules to decide which move you should make given
the current state of the board. It is split up into 3 sections: ban rules, try
rules, and the fall back sequence. Each rule is made up of a condition and a
move direction, if the condition is met they define some action to take.
Conditions test if some pattern is present on the board. The action to take
depends on whether it is a ban rule or a try rule. Every time a move is made and
the state of the board changes, you go through the strategy from the start.

<h3 class="mt-4 mb-1 text-xl font-semibold">Ban Rules</h3>
This is a list of ban rules. The order in which you process this list is not
important. You will need to go through the ban rules checking if the condition
is met and what move to ban. Banning a move means you do not make that move even
if a try rule says you should.

<h3 class="mt-4 mb-1 text-xl font-semibold">Try Rules</h3>
Unlike the ban rules the ordering of the try rules is important, you should
start at the first rule in the list. If the condition is met you will "try" to
carry out the move. There are 2 cases where this "try" fails:
<ol class="ml-4 list-decimal list-inside">
  <li>If the move has been banned.</li>
  <li>If the move is not possible (none of the tiles move).</li>
</ol>
If the condition for the rule is not met or it fails, move on to the next rule
in the list. If all the rules in the list have been considered and no move has
been made move on to the fall back sequence.

<h3 class="mt-4 mb-1 text-xl font-semibold">Fall back sequence</h3>
As with the try rules the ordering is important. Simple go through the list and
"try" (as with the try rules) to make each move, the same failure cases apply.
Most of the time by the end of this list you will have made a move. If not then
go back through the list, disregarding the moves that are banned, and make the
first move that is possible.

<h2 class="mt-4 mb-1 text-2xl font-semibold">The Strategy</h2>

<h3 class="mt-4 mb-1 text-xl font-semibold">Ban Rules</h3>
{#each rules as {name, description, examples}}
<Rule {name} {description} {examples} />
{/each}

<!--
<figure>
  <img alt="Success Kid" src="successkid.jpg" />
  <figcaption>Hve fun with Sapper!</figcaption>
</figure>
-->
