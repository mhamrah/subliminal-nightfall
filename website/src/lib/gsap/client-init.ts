import { gsap } from './init';

/**
 * Initialize all GSAP animations on client side
 */
export function initAnimations() {
  console.log('🎬 Starting basic initialization...');

  try {
    // Simple fade-in for body - that's it!
    const body = document.querySelector('body');
    if (body) {
      gsap.fromTo(body,
        { opacity: 0 },
        { opacity: 1, duration: 0.5, ease: 'power2.out' }
      );
      console.log('✅ Page loaded');
    }
  } catch (error) {
    console.error('❌ Error:', error);
  }
}
