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


  // Track where user came from
  let prevPath = '/LogView';
  if (typeof window !== 'undefined') {
    const ref = document.referrer;
    if (ref && ref.startsWith(window.location.origin)) {
      const url = new URL(ref);
      prevPath = url.pathname || '/LogView';
    } else if (window.history.state && window.history.state.idx > 0) {
      prevPath = document.referrer || '/LogView';
    }
  }

  function exitOnboarding() {
    goto(prevPath || '/LogView');
  }

  function handleKey(e: KeyboardEvent) {
    if (e.key === 'ArrowLeft') goLeft();
    if (e.key === 'ArrowRight') goRight();
    if (e.key === 'Escape') exitOnboarding();
  }

  onMount(() => {
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  });
</script>

<main class="onboarding" on:click={(e) => {
  // Only exit if click is outside the slide
  if ((e.target as HTMLElement).classList.contains('onboarding')) exitOnboarding();
}}>
  <div class="slide" on:click|self={(e) => {
    const x = (e as MouseEvent).offsetX;
    const w = (e.target as HTMLElement).clientWidth;
    if (x < w * 0.3) goLeft();
    else if (x > w * 0.7) goRight();
  }}>
    <div class="slide-content">
      <h1>Welcome</h1>
      <div class="slide-number">Page {current + 1} / {total}</div>
    </div>
    <div class="slide-nav left">&#8592;</div>
    <div class="slide-nav right">&#8594;</div>
  </div>
</main>

<style>
.onboarding {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--background, #fffbe9);
}
.slide {
  position: relative;
  width: 420px;
  height: 320px;
  background: #fff;
  border-radius: 16px;
  box-shadow: 0 4px 24px rgba(0,0,0,0.08);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}
.slide-content {
  text-align: center;
}
.slide-number {
  margin-top: 1.2em;
  color: #a95e45;
  font-size: 1.1em;
}
.slide-nav {
  position: absolute;
  top: 50%;
  font-size: 2.2em;
  color: #CC785C;
  user-select: none;
  pointer-events: none;
}
.slide-nav.left {
  left: 18px;
  transform: translateY(-50%);
}
.slide-nav.right {
  right: 18px;
  transform: translateY(-50%);
}
</style>
