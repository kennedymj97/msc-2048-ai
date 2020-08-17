<script>
  import Nav from '../components/Nav.svelte';

  export let segment;
</script>

<style>
  main {
    position: relative;
    background-color: white;
    box-sizing: border-box;
	max-width: 1024px;
	margin: 0 auto;
  }
</style>

<Nav {segment} />

<main>
  <slot />
</main>
