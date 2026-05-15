<script lang="ts">
  const version = import.meta.env.VITE_APP_VERSION;
</script>

<main>
  <h1>Token Tach</h1>
  <p>AI spend monitor — v{version}</p>
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    font-family: system-ui, -apple-system, sans-serif;
    color: #e0e0e0;
    background: #1a1a2e;
  }

  h1 {
    font-size: 2rem;
    margin-bottom: 0.5rem;
  }

  p {
    opacity: 0.6;
  }
</style>
