<script>
  import Game from '../components/2048.svelte';
  import { onMount, onDestroy } from 'svelte';

  onMount(() => {
	  console.log("mounting ai");
  });

  onDestroy(() => {
	  console.log("destroying ai");
  });
</script>

<Game name="ai" isAi={true}></Game>
