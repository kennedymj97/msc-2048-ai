<script>
  export let segment;
</script>

<style>
  nav {
    border-bottom: 1px solid rgba(255, 62, 0, 0.1);
    font-weight: 300;
    padding: 0 1em;
	z-index: 200;
  }

  ul {
    margin: 0;
    padding: 0;
	text-align:center;
  }

  /* clearfix */
  ul::after {
    content: '';
    display: block;
    clear: both;
  }

  li {
    display: inline-block;
  }

  [aria-current] {
    position: relative;
    display: inline-block;
  }

  [aria-current]::after {
    position: absolute;
    content: '';
    width: calc(100% - 1em);
    height: 2px;
    background-color: rgb(255, 62, 0);
    display: block;
    bottom: -1px;
  }

  a {
    text-decoration: none;
    padding: 0.4em 0.5em;
    display: block;
  }
</style>

<nav class="mb-4">
  <ul>
    <li>
      <a aria-current={segment === undefined ? 'page' : undefined} href=".">
        Introduction
      </a>
    </li>
    <li>
      <a aria-current={segment === '2048' ? 'page' : undefined} href="2048">
        2048
      </a>
    </li>
    <li>
      <a aria-current={segment === 'tutorial' ? 'page' : undefined} href="tutorial">
        Tutorial
      </a>
    </li>
    <li>
      <a aria-current={segment === 'testing' ? 'page' : undefined} href="testing">
        Testing
      </a>
    </li>
  </ul>
</nav>
