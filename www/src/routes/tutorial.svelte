<script>
  import Rule from '../components/rule.svelte';

  let ban_rules = ['Ban move up if left column not locked'];
  let try_rules = [
    'Try move left if moves largest tile to bottom left corner',
    'Try move up if produces potential merge in the left/right direction',
    'Try move down if produces potential merge in the left/right direction',
    'Try move down if creates monotonic middle top row',
  ];
  let fallback = ['Left', 'Down', 'Up', 'Right'];

  let ban_rule_info = [
    {
      name: 'Ban move up if left column not locked',
      description:
        "If the leftmost column is full and no merges are possible it is 'locked'. Making a move in the vertical direction will not change the state of the column. For this rule, if the leftmost column is NOT locked the move up is banned.",
      examples: [
        {
          imagePath: './tutorial_images/ban/not_full.png',
          result: 'Ban move up.',
          reason:
            'The leftmost column is not full. A move in the vertical direction (up or down) will shift the tiles.',
        },
        {
          imagePath: './tutorial_images/ban/full_no_merge.png',
          result: 'Do not ban. Go to the first try rule.',
          reason:
            'The leftmost column is full of tiles and no merge is possible in the vertical direction. The tiles will not shift if a move is made in the vertical direction.',
        },
        {
          imagePath: './tutorial_images/ban/full_merge.png',
          result: 'Ban move up.',
          reason:
            'The left column is full of tiles but a merge is possible. A move in the vertical direction will cause the merge to happen and the tiles will shift.',
        },
      ],
    },
  ];
  let try_rule_info = [
    {
      name: 'Try move left if moves largest tile to bottom left corner',
      description:
        'If the largest tile is not in the bottom left corner and the move left will make the bottom left corner contain the largest tile, try the move left. If there are multiple largest tiles, only one of them needs to be in the bottom left corner.',
      examples: [
        {
          imagePath: './tutorial_images/corner/already_in_corner.png',
          result: 'Skip to try rule 2.',
          reason: 'The largest tile is already in the bottom left corner.',
        },
        {
          imagePath: './tutorial_images/corner/slides.png',
          result: 'Try move left.',
          reason:
            'The largest tile is currently not in the bottom left corner. Making the move left will shift the largest tile to the bottom left corner.',
        },
        {
          imagePath: './tutorial_images/corner/merges.png',
          result: 'Try move left.',
          reason:
            'The largest tile is currently not in the bottom left corner. Making the move left will merge tiles and make the bottom left corner contain one of the largest tiles.',
        },
      ],
    },
    {
      name: 'Try move up if produces potential merge in the left/right direction',
      description:
        'This rule checks if moving up will produce a potential merge in the left or right direction. You will try the move up when there is currently no merge possible in the left or right direction, and making the move up creates a possible move in the left or right direction. Make sure to check if the move up is banned. The rule "Try move down if produces potential merge in the left/right direction" has the same conditions but considers the move down instead of up.',
      examples: [
        {
          imagePath:
            './tutorial_images/produces_merge/does_no_produce_merge.png',
          result: 'Skip to the next try rule.',
          reason:
            'Making the move up will not produce any possible merges in the left or right direction.',
        },
        {
          imagePath: './tutorial_images/produces_merge/produces_merge.png',
          result: 'Try move up.',
          reason:
            'There is currently no merge possible in the left or right direction. Moving up makes a merge possible in the left or right direction.',
        },
        {
          imagePath:
            './tutorial_images/produces_merge/merge_already_possible.png',
          result: 'Skip to the next try rule.',
          reason:
            'There is already a merge possible in the left or right direction.',
        },
      ],
    },
    {
      name: 'Try move down if creates monotonic middle top row',
      description:
        'The middle top row is the one below the top row. Monotonicity means the values for the tiles in the row are decreasing/equal all the way across the row. A fully empty row does not count as being monotonic. It can be monotonic either left to right or right to left. If the middle top row is not currently monotonic and making the move down makes it monotonic, then try making the move down. An empty tile counts as 0. This rule often applys when there are tiles on the middle top row seperated by empty tiles, in this case the row is not monotonic. If after making the move down there are no longer tiles seperated by empty tiles you should try making the move down.',
      examples: [
        {
          imagePath: './tutorial_images/monotonic/already_monotonic.png',
          result: 'Go to fall back sequence.',
          reason: 'The middle top row is already monotonic.',
        },
        {
          imagePath:
            './tutorial_images/monotonic/not_monotonic_and_not_create.png',
          result: 'Go to fall back sequence.',
          reason:
            'The middle top row is not monotonic but making the move down will not make it monotonic.',
        },
        {
          imagePath:
            './tutorial_images/monotonic/not_monotonic_create_monotonic.png',
          result: 'Try move down.',
          reason:
            'The middle top row is not monotonic. Making the move down will make the middle top row monotonic.',
        },
        {
          imagePath: './tutorial_images/monotonic/empty.png',
          result: 'Try move down.',
          reason:
            'The midddle top row is not monotonic due to the empty tile. Making the move down will make the middle top row monotonic. Note: this would cause the rule "try move down if produces merge in the left/right direction" to skip as in the current state of the board there is already a left/right merge possible.',
        },
      ],
    },
  ];
</script>

<svelte:head>
  <title>Introduction</title>
</svelte:head>

<div class="flex flex-row justify-center mb-4">
  <a class="font-semibold underline" href="testing">Go to testing &rarr;</a>
</div>
<h1 class="mt-4 mb-2 text-3xl font-semibold">Tutorial</h1>
<p class="mb-2 text-justify">
  This tutorial will describe how to follow the strategy. Please read it before
  doing the testing. You can come back to this page at any time during the
  testing, your progress in the game will be saved.
</p>

<p>
  This flowchart is designed to help you follow the strategy when doing the
  test. Refer back to it if you are not sure what move to make. Before reading
  the flowchart it may be useful to go through the rest of the tutorial below.
</p>
<img src="./tutorial_images/strategy.svg" alt="flow diagram for the strategy" />

<h2 class="mt-4 mb-2 text-2xl font-semibold">What is a strategy?</h2>
<p class="mb-2 text-justify">
  A strategy consists of a set of rules to decide which move you should make.
  Each rule is made up of a condition and a move direction, if the condition is
  met they define some action to take. Conditions involve checking if some
  pattern is present on the board. Some conditions only consider the current
  state of the board, others consider the current state of the board and what
  the state of the board will be after making a certain move. The action to take
  depends on whether it is a ban rule or a try rule. Every time a move is made
  and the state of the board changes, you go through the strategy from the
  start.
</p>
<p class="text-justify">
  The strategy is split up into 3 sections: ban rules, try rules, and the fall
  back sequence.
</p>

<h3 class="mt-4 mb-2 text-xl font-semibold">Ban Rules</h3>
<p class="text-justify">
  You will need to go through the ban rules checking if the condition is met and
  what move to ban. Banning a move means you do not make that move even if a try
  rule says you should. You will have to remember the moves banned. Therefore,
  if any of the try rules are based on a move that is banned you can immediately
  skip that try rule and move on to the next one.
</p>

<h3 class="mt-4 mb-2 text-xl font-semibold">Try Rules</h3>
<p class="mb-2 text-justify">
  You start at the first rule in the list. If the condition is met you will
  "try" to carry out the move.
</p>
<span>There are 2 cases where this "try" fails:</span>
<ol class="mb-2 ml-4 list-decimal list-outside">
  <li>If the move has been banned.</li>
  <li>If the move is not possible (none of the tiles move).</li>
</ol>
<p class="text-justify">
  If the condition for the rule is not met or it fails, move on to the next rule
  in the list. If all the rules in the list have been considered and no move has
  been made move on to the fall back sequence. The flowchart goes into exactly
  what you should check for each try rule. The examples and descriptions below
  may also help. For these try rules you have to consider the current state of
  the board and what the state of the board will be after making the move before
  deciding whether to "try" making the move or to skip to the next rule.
</p>

<h3 class="mt-4 mb-2 text-xl font-semibold">Fall back sequence</h3>
<p class="text-justify">
  Simply go through the list and "try" (as with the try rules) to make each
  move, the same failure cases apply. Remember to skip a move if it has been
  banned. Most of the time by the end of this list you will have made a move. If
  not then go back through the list, disregarding the moves that are banned, and
  make the first move that is possible.
</p>

<div class="flex flex-col">
  <h2 class="mt-4 mb-2 text-2xl font-semibold">The Strategy</h2>
  <span class="font-semibold">Ban rules:</span>
  <ul class="pl-5 mb-2 list-disc list-outside">
    {#each ban_rules as ban_rule}
    <li>{ban_rule}</li>
    {/each}
  </ul>
  <span class="font-semibold">Try rules:</span>
  <ol class="pl-4 mb-2 list-decimal list-outside">
    {#each try_rules as try_rule}
    <li>{try_rule}</li>
    {/each}
  </ol>
  <span class="font-semibold">Fall back sequence:</span>
  <ol class="pl-4 mb-2 list-decimal list-outside">
    {#each fallback as direction}
    <li>{direction}</li>
    {/each}
  </ol>
</div>

<h3 class="mt-4 mb-2 text-xl font-semibold">Ban Rules</h3>
{#each ban_rule_info as {name, description, examples}}
<Rule {name} {description} {examples}></Rule>
{/each}

<h3 class="mt-4 mb-2 text-xl font-semibold">Try Rules</h3>
{#each try_rule_info as {name, description, examples}}
<Rule {name} {description} {examples}></Rule>
{/each}

<div class="flex flex-row justify-center mb-4">
  <a class="font-semibold underline" href="testing">Go to testing &rarr;</a>
</div>
