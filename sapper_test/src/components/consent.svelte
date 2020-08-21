<script>
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  let hasChecked = false;
  export let onAccept = () => alert('Not implemented');

  function handleAccept() {
    onAccept();
    dispatch('message', {});
  }
</script>

<div
  class="fixed top-0 left-0 z-50 flex items-center justify-center w-screen h-screen p-10 bg-gray-400 bg-opacity-75"
>
  <div
    class="flex flex-col px-12 pt-4 pb-6 text-justify bg-white rounded-md"
    style="max-width: 1024px;"
  >
    <h1 class="self-center mb-2 text-3xl font-semibold">Consent Form</h1>
    <h2 class="mb-3 text-lg font-semibold">
      Project: Learning Interpretable Strategies for the 2048 Videogame
    </h2>
    <h2 class="mb-1 text-lg font-semibold">Purpose of the research</h2>
    <p class="mb-3 leading-5">
      This research aims to use AI methods to produce strategies people can
      understand. Participation will help understand how well the strategies can
      be understood/followed by people.
    </p>
    <h2 class="mb-1 text-lg font-semibold">Nature of participation</h2>
    <p class="leading-5">
      Paricipation in the research is voluntary and you will not be required to
      provide any personal data. We will collect an anonymous record of the
      games of 2048 that you play during the study.
    </p>
    <h2 class="mb-1 text-lg font-semibold">Participants tasks</h2>
    <p class="mb-3 leading-5">
      You will be asked to play the game of 2048 and follow a strategy found
      using AI methods. We will collect a record of the games you play during
      the study. This data will not contain any information that could identify
      you.
    </p>
    <h2 class="mb-1 text-lg font-semibold">
      Benefits and risks of the research
    </h2>
    <p class="mb-3 leading-5">
      Your pariticipation will help us understand how well you could follow the
      strategies. There is no risk in pariticipating in the study.
    </p>
    <h2 class="mb-1 text-lg font-semibold">Use of your data</h2>
    <p class="mb-3 leading-5">
      The data will be used in a project report and other information about the
      project. It may also be published to support future research. The data
      collected will be anonymous, no information that can identify you will be
      collected.
    </p>
    <h2 class="mb-1 text-lg font-semibold">Future use of your data</h2>
    <p class="mb-3 leading-5">
      The data we collect may be archived and reused in future for purposes that
      are in the public interest, or for historical, scientific or statistical
      purposes. This will allow other to view the raw data themselves and be
      able to verigy conslusions made in the project or derive new conclusions.
      The data may be stored on a cloud provider or a researcher's computer and
      made available online.
    </p>
    <h2 class="mb-1 text-lg font-semibold">
      Procedure to withdraw from the research
    </h2>
    <p class="mb-3 leading-5">
      You may withdraw from the study at any time before or during
      pariticipation and do not have to give reasons for why you no longer want
      to take part. Once submitted, the records of the games you produce cannot
      be withdrawn due to the data being anonymous. To withdraw close the
      webpage and no further data will be collected.
    </p>
    <h2 class="mb-1 text-lg font-semibold">
      Contact details of the ethics committee
    </h2>
    <p class="mb-3 leading-5">
      If you wish to file a complaint or exercise you rights you can contact the
      Ethics Committe at the following address: cs-ethicsadmin@cs.nott.ac.uk
    </p>
    <label class="self-center mt-3">
      <input type="checkbox" bind:checked="{hasChecked}" />
      I have read the consent form and understand what participating in the
      study involves
    </label>
    <button
      class="self-center px-8 py-2 mt-4 font-semibold text-white bg-green-500 rounded disabled:bg-opacity-25 disabled:cursor-default"
      disabled="{!hasChecked}"
      on:click="{handleAccept}"
    >
      Accept
    </button>
  </div>
</div>
