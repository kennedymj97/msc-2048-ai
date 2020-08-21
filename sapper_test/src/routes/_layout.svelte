<script>
  import Nav from '../components/Nav.svelte';
  import Consent from '../components/consent.svelte';

  export let segment;
  let hasCompletedConsentForm;
  const checkIfConsentCompleted = () => hasCompletedConsentForm = getCookie("consent") != null;

  let onAccept = () => {
	  setCookie("consent", 30);
  }

  if (process.browser) {
	  checkIfConsentCompleted();
  }

  function setCookie(name,value,days) {
    var expires = "";
    if (days) {
      var date = new Date();
      date.setTime(date.getTime() + (days*24*60*60*1000));
      expires = "; expires=" + date.toUTCString();
    }
    document.cookie = name + "=" + (value || "")  + expires + "; path=/";
  }

  function getCookie(name) {
    var nameEQ = name + "=";
    var ca = document.cookie.split(';');
    for(var i=0;i < ca.length;i++) {
      var c = ca[i];
      while (c.charAt(0)==' ') c = c.substring(1,c.length);
      if (c.indexOf(nameEQ) == 0) return c.substring(nameEQ.length,c.length);
    }
    return null;
  }

  function eraseCookie(name) {   
    document.cookie = name +'=; Path=/; Expires=Thu, 01 Jan 1970 00:00:01 GMT;';
  }

</script>

<style>
  main {
    position: relative;
    background-color: white;
    box-sizing: border-box;
	max-width: 1024px;
	margin: 50px auto 50px auto;
  }

  @media(max-width: 500px) {
	main {
	  margin: 90px 25px 50px 25px;
	}
  }

  @media(max-width: 1124px) and (min-width: 500px) {
	main {
	  margin: 50px 50px 50px 50px;
	}
  }
</style>

{#if !hasCompletedConsentForm}
<Consent {onAccept} on:message={checkIfConsentCompleted} />
{:else}
<Nav {segment} />

<main>
  <slot />
</main>
{/if}
