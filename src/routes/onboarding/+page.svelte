<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let current = 0;
  const slides = [
    {
      title: "Welcome to Quiet Wins",
      content: "Track your daily achievements and see your progress over time. Let's get started!",
      icon: "🏆"
    },
    {
      title: "Quick Win Logging",
      content: "Press Cmd+Alt+Shift+W anywhere to instantly log a win. Tags are automatically suggested!",
      icon: "⚡"
    },
    {
      title: "Soft Delete & Trash",
      content: "Deleted wins go to Trash for 48 hours. Click the 🗑️ button to restore them anytime!",
      icon: "🗑️"
    },
    {
      title: "Visualize Your Progress",
      content: "See tag relationships in the Graph View and track stats in Recap View with 7-day and 30-day summaries.",
      icon: "📊"
    },
    {
      title: "Customize Everything",
      content: "Choose themes, set notification times, enable privacy lock, and customize your experience in Settings.",
      icon: "⚙️"
    }
  ];
  const total = slides.length;

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

<main class="onboarding" role="presentation" on:click={(e) => {
  // Only exit if click is outside the slide
  if ((e.target as HTMLElement).classList.contains('onboarding')) exitOnboarding();
}}>
  <div 
    class="slide" 
    role="dialog"
    aria-labelledby="slide-title"
    aria-describedby="slide-content"
    on:click|self={(e) => {
      const x = (e as MouseEvent).offsetX;
      const w = (e.target as HTMLElement).clientWidth;
      if (x < w * 0.3) goLeft();
      else if (x > w * 0.7) goRight();
    }}
  >
    <div class="slide-content">
      <div class="slide-icon" aria-hidden="true">{slides[current].icon}</div>
      <h1 id="slide-title">{slides[current].title}</h1>
      <p id="slide-content">{slides[current].content}</p>
      <div class="slide-number" aria-label="Page {current + 1} of {total}">
        {current + 1} / {total}
      </div>
      <div class="slide-controls">
        <button 
          class="nav-btn"
          on:click={goLeft} 
          disabled={current === 0}
          aria-label="Previous slide"
        >
          ← Previous
        </button>
        <button 
          class="nav-btn primary"
          on:click={goRight}
          aria-label={current === total - 1 ? "Get started" : "Next slide"}
        >
          {current === total - 1 ? "Get Started 🚀" : "Next →"}
        </button>
      </div>
      <button class="skip-btn" on:click={exitOnboarding} aria-label="Skip onboarding">
        Skip
      </button>
    </div>
    <div class="slide-nav left" aria-hidden="true">&#8592;</div>
    <div class="slide-nav right" aria-hidden="true">&#8594;</div>
  </div>
</main>

<style>
  .onboarding {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
  }
  
  .slide {
    background: var(--bg-primary);
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    max-width: 500px;
    width: 90%;
    padding: 3rem 2rem;
    position: relative;
    user-select: none;
    cursor: pointer;
  }
  
  .slide-content {
    text-align: center;
  }
  
  .slide-icon {
    font-size: 4rem;
    margin-bottom: 1rem;
    line-height: 1;
  }
  
  .slide-content h1 {
    margin: 0 0 1rem 0;
    font-size: 1.75rem;
    color: var(--text-primary);
  }
  
  .slide-content p {
    color: var(--text-secondary);
    font-size: 1rem;
    line-height: 1.6;
    margin: 0 0 2rem 0;
  }
  
  .slide-number {
    font-size: 0.85rem;
    color: var(--text-tertiary);
    margin-bottom: 1.5rem;
  }
  
  .slide-controls {
    display: flex;
    gap: 0.75rem;
    justify-content: center;
  }
  
  .nav-btn {
    padding: 0.65rem 1.5rem;
    border-radius: 8px;
    border: 1px solid var(--border);
    background: var(--bg-secondary);
    color: var(--text-primary);
    cursor: pointer;
    font-size: 0.95rem;
    transition: all 0.2s;
  }
  
  .nav-btn:hover:not(:disabled) {
    background: var(--bg-hover);
    transform: translateY(-1px);
  }
  
  .nav-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  
  .nav-btn.primary {
    background: var(--accent);
    color: white;
    border-color: var(--accent);
  }
  
  .nav-btn.primary:hover:not(:disabled) {
    background: var(--accent-hover);
    border-color: var(--accent-hover);
  }
  
  .skip-btn {
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: transparent;
    border: none;
    color: var(--text-tertiary);
    cursor: pointer;
    font-size: 0.9rem;
    padding: 0.5rem;
  }
  
  .skip-btn:hover {
    color: var(--text-primary);
  }
  
  .slide-nav {
    position: absolute;
    top: 50%;
    transform: translateY(-50%);
    font-size: 2rem;
    color: var(--text-tertiary);
    opacity: 0.3;
    pointer-events: none;
  }
  
  .slide-nav.left { left: 1rem; }
  .slide-nav.right { right: 1rem; }
</style>

