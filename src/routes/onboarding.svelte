<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let current = 0;
  const total = 5;

  function goLeft() {
    if (current > 0) current--;
  }
  function goRight() {
    if (current < total - 1) current++;
    else {
      // Mark onboarding as complete and go to main app
      localStorage.setItem('qw-onboarding-complete', '1');
      goto('/LogView');
    }
  }

  function handleKey(e: KeyboardEvent) {
    if (e.key === 'ArrowLeft') goLeft();
    if (e.key === 'ArrowRight') goRight();
  }

  onMount(() => {
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  });
</script>

<main class="onboarding">
  <div class="slide" on:click|self={(e) => {
