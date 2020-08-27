<p class="mb-1 font-semibold">After making a move return to step 1.</p>
<ol class="ml-4 text-justify list-decimal list-outside">
  <li class="mb-1">
    If the bottom left corner does not contain a tile with a value equal to the
    largest on the grid and after moving left it will contain a tile with a
    value equal to the largest on the board, make the move left.
  </li>
  <li class="mb-1">
    Go to step 4 if moving up will change the left column. Otherwise continue to
    step 3.
  </li>
  <li class="mb-1">
    If there is no merge possible in the left direction and after moving up a
    merge will be possible in the left direction, then move up.
  </li>
  <li class="mb-1">
    If there is no merge possible in the left direction and after moving down a
    merge will be possible in the left direction, then move down.
  </li>
  <li class="mb-1">
    If the row below the top row is currently not monotonic and after moving
    down will be monotonic then move down.
  </li>
  <li>
    <span>Make the first move possible:</span>
    <ol class="ml-4 list-decimal list-outside">
      <li>Left</li>
      <li>Down</li>
      <li>
        Up if it will not change the left column or is the only possible move
      </li>
      <li>Right</li>
    </ol>
  </li>
</ol>
