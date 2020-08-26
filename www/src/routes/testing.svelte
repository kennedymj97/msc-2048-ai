<script>
  import { onMount, onDestroy } from 'svelte';
  import Game from '../components/2048.svelte';
  import {v4 as uuidv4 } from 'uuid';

  let aiMove;
  let showAiMove = false;
  let makeMove;

  let ban_rules = ['Ban move up if left column not locked'];
  let try_rules = [
    'Try move left if moves largest tile to bottom left corner',
    'Try move up if produces potential merge in the left/right direction',
    'Try move down if produces potential merge in the left/right direction',
    'Try move down if creates monotonic middle top row',
  ];
  let fallback = ['Left', 'Down', 'Up', 'Right'];

  function toggleShowMove() {
    if (showAiMove) {
      makeMove(aiMove);
    }
    showAiMove = !showAiMove;
  }

  onMount(() => {
    if (window.localStorage) {
	  if (!localStorage.getItem("id")) {
		localStorage['id'] = uuidv4();
	  }
      if (!localStorage.getItem('firstLoad')) {
        localStorage['firstLoad'] = true;
        window.location.reload();
      }
    }
  });

  onDestroy(() => {
    if (window.localStorage) {
      localStorage.removeItem('firstLoad');
    }
  });
</script>

<div class="flex flex-col items-center lg:flex-row lg:items-start">
  <div class="flex flex-col px-6">
    <p>
      <strong>
        All moves made when the answer is hidden will be used in the study.
        Please check your answers before making a move until you are confident
        you understand the strategy.</strong
      ><br />If you are unsure, you can check the tutorial page. Progress made
      in the game will be saved.
    </p>
    <div class="flex flex-row justify-center mb-4">
      <a class="font-semibold underline" href="tutorial"
        >&larr; Go to tutorial</a
      >
    </div>
    <hr />
	<p class="mb-2 italic">Please check flow chart before making a move until fully confident.</p>
	<p class="mb-2 italic"><span class="font-bold">MERGES!</span> If considering the state after making a move remember to merge the tiles.</p>
    <span class="font-bold">Ban rules:</span>
    <ul class="pl-5 mb-2 list-disc list-outside">
      {#each ban_rules as ban_rule}
      <li>{ban_rule}</li>
      {/each}
    </ul>
    <span class="font-bold">Try rules:</span>
    <ol class="pl-4 mb-2 list-decimal list-outside">
      {#each try_rules as try_rule}
      <li>{try_rule}</li>
      {/each}
    </ol>
    <span class="font-bold">Fall back sequence:</span>
    <ol class="pl-4 mb-2 list-decimal list-outside">
      {#each fallback as direction}
      <li>{direction}</li>
      {/each}
    </ol>
    <hr />
    <div class="flex flex-row items-center">
      {#if showAiMove}
      <button
        class="px-4 py-2 mb-2 font-semibold text-white bg-green-400 rounded"
        on:click="{toggleShowMove}"
      >
        Hide answer
      </button>
      <span
        class="ml-6 text-lg font-semibold uppercase"
        id="ai-move"
        >{aiMove}</span
      >
      {:else}
      <button
        class="self-center px-4 py-2 mb-2 font-semibold text-white bg-green-400 rounded"
        on:click="{toggleShowMove}"
      >
        Check answer
      </button>
      {/if}
    </div>
    <p class="mt-2">
      Use this button to check your answers when practicing. Try to go through
      the strategy before checking the answer. After checking we recommend you
      hide the answer,
      <strong
        >the computer will make the move for you when you hide the answer to
        prevent cheating</strong
      >. If you are showing the answer the data will <strong>NOT</strong> be
      inluded in the study.
      <strong>When you start the test make sure the answers are hidden.</strong>
    </p>
  </div>
  <Game
    name="testing"
    isTesting="{true}"
    bind:aiMove="{aiMove}"
    bind:makeMove="{makeMove}"
  ></Game>
  <div></div>
</div>
