<script>
  export let name;
  export let description;
  export let examples;
</script>

<h4 class="mt-4 mb-2 text-lg font-semibold">{name}</h4>

<p class="text-justify">{description}</p>

<h5 class="mt-4 mb-2 font-semibold">Examples:</h5>
<div class="flex flex-row flex-wrap mb-8">
  {#each examples as { imagePath, result, reason }}
  <div class="flex flex-col my-3 mr-10" style="width: 300px">
    <img class="mb-2" style="height: 300px" src="{imagePath}" />
	<span class="text-justify"><span class="font-semibold">Result: </span>{result}</span>
	<span class="text-justify"><span class="font-semibold">Reason: </span>{reason}</span>
  </div>
  {/each}
</div>
