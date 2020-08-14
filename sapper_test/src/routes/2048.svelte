<script>
  import { onMount } from 'svelte';

  let gameManager;
  let hexString;
  let wasm;

  onMount(async () => {
    const { default: GameManager } = await import('../js/game_manager.js');
    const { default: KeyboardInputManager } = await import(
      '../js/keyboard_input_manager.js'
    );
    const { default: HTMLActuator } = await import('../js/html_actuator.js');
    const { default: LocalStorageManager } = await import(
      '../js/local_storage_manager.js'
    );
    wasm = await import('msc-2048-ai-wasm');

    // Wait till the browser is ready to render the game (avoids glitches)
    window.requestAnimationFrame(function () {
      gameManager = new GameManager(
        4,
        KeyboardInputManager,
        HTMLActuator,
        LocalStorageManager
      );
      hexString = gameManager.gridCellsToHexString(gameManager.grid.cells);
    });
  });

  function handleKeydown(event) {
    if (!gameManager.over) {
      hexString = gameManager.gridCellsToHexString(gameManager.grid.cells);
      console.log(wasm.wasm_test_fn());
      console.log(hexString + 'blah');
    }
  }
</script>

<svelte:head>
  <meta charset="utf-8" />
  <title>2048</title>

  <link href="/2048.css" rel="stylesheet" type="text/css" />
  <link rel="shortcut icon" href="/2048-app/favicon.ico" />
  <link rel="apple-touch-icon" href="/2048-app/meta/apple-touch-icon.png" />
  <link
    rel="apple-touch-startup-image"
    href="meta/apple-touch-startup-image-640x1096.png"
    media="(device-width: 320px) and (device-height: 568px) and
    (-webkit-device-pixel-ratio: 2)" />
  <!-- iPhone 5+ -->
  <link
    rel="/2048-app/apple-touch-startup-image"
    href="/2048-app/meta/apple-touch-startup-image-640x920.png"
    media="(device-width: 320px) and (device-height: 480px) and
    (-webkit-device-pixel-ratio: 2)" />
  <!-- iPhone, retina -->
  <meta name="apple-mobile-web-app-capable" content="yes" />
  <meta name="apple-mobile-web-app-status-bar-style" content="black" />

  <meta name="HandheldFriendly" content="True" />
  <meta name="MobileOptimized" content="320" />
  <meta
    name="viewport"
    content="width=device-width, target-densitydpi=160dpi, initial-scale=1.0,
    maximum-scale=1, user-scalable=no, minimal-ui" />
</svelte:head>

<svelte:window on:keydown={handleKeydown} />

<div class="container">
  <div class="heading">
    <h1 class="title">2048</h1>
    <div class="scores-container">
      <div class="score-container">0</div>
      <div class="best-container">0</div>
    </div>
  </div>

  <div class="above-game">
    <p class="game-intro">
      Join the numbers and get to the
      <strong>2048 tile!</strong>
    </p>
    <a class="restart-button">New Game</a>
  </div>

  <div class="game-container">
    <div class="game-message">
      <p />
      <div class="lower">
        <a class="keep-playing-button">Keep going</a>
        <a class="retry-button">Try again</a>
      </div>
    </div>

    <div class="grid-container">
      <div class="grid-row">
        <div class="grid-cell" />
        <div class="grid-cell" />
        <div class="grid-cell" />
        <div class="grid-cell" />
      </div>
      <div class="grid-row">
        <div class="grid-cell" />
        <div class="grid-cell" />
        <div class="grid-cell" />
        <div class="grid-cell" />
      </div>
      <div class="grid-row">
        <div class="grid-cell" />
        <div class="grid-cell" />
        <div class="grid-cell" />
        <div class="grid-cell" />
      </div>
      <div class="grid-row">
        <div class="grid-cell" />
        <div class="grid-cell" />
        <div class="grid-cell" />
        <div class="grid-cell" />
      </div>
    </div>

    <div class="tile-container" />
  </div>

  <p class="game-explanation">
    <strong class="important">How to play:</strong>
    Use your
    <strong>arrow keys</strong>
    to move the tiles. When two tiles with the same number touch, they
    <strong>merge into one!</strong>
  </p>
  <hr />
  <p>
    <strong class="important">Note:</strong>
    This site is the official version of 2048. You can play it on your phone via
    <a class="link" href="http://git.io/2048">http://git.io/2048.</a>
    All other apps or sites are derivatives or fakes, and should be used with
    caution.
  </p>
  <hr />
  <p>
    Created by
    <a class="link" href="http://gabrielecirulli.com" target="_blank">
      Gabriele Cirulli.
    </a>
    Based on
    <a
      class="link"
      href="https://itunes.apple.com/us/app/1024!/id823499224"
      target="_blank">
      1024 by Veewo Studio
    </a>
    and conceptually similar to
    <a class="link" href="http://asherv.com/threes/" target="_blank">
      Threes by Asher Vollmer.
    </a>
  </p>
</div>
