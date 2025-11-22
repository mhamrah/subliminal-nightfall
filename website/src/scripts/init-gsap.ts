import { initAnimations } from '../lib/gsap/client-init';

// Initialize GSAP animations after DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initAnimations);
} else {
  // Small delay to ensure all components are mounted
  setTimeout(initAnimations, 50);
}

