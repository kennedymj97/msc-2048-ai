<script>
  import { onMount, onDestroy } from 'svelte';
  import Game from '../components/2048.svelte';

  let aiMove;
  let showAiMove = false;
  let makeMove;

  let ban_rules = ['Ban move up if left column not locked'];
  let try_rules = [
    'Try move left if moves largest tile to bottom left corner',
    'Try move up if produces merge',
    'Try move down if produces merge',
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
  <div class="flex flex-col max-w-sm px-6">
    <p>
      <strong>
        All moves made when the answer is hidden will be used in the study.
        Please do not start making moves with the answer hidden until you are
        confident you understand the strategy.</strong
	><br />If you are unsure, you can check the tutorial page. Progress made in the
      game will be saved.
    </p>
    <div class="flex flex-row justify-center mb-4">
      <a class="font-semibold underline" href="tutorial">&larr; Go to tutorial</a>
    </div>
    <hr />
    <span class="mb-2 text-lg font-semibold">Strategy</span>
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
    <hr />
    <div class="flex flex-col">
      {#if showAiMove}
      <button
        class="self-center px-4 py-2 mb-2 font-semibold text-white bg-green-400 rounded"
        on:click="{toggleShowMove}"
      >
        Hide answer
      </button>
      <span
        class="self-center mt-2 text-lg font-semibold uppercase"
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
      the strategy before checking the answer. When you are confident hide the
      answer and stop checking it. If you are showing the answer the data will
      <strong>NOT</strong> be inluded in the study.
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
