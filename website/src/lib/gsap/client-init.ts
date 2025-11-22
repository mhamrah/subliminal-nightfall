import { initScrollSmoother, ScrollTrigger, gsap } from './init';
import { createScrollProgress } from './scroll-progress';
import { initHeroAnimations } from './hero-animations';
import { initColorPaletteAnimations } from './color-palette-animations';
import { initCodeExamplesAnimations } from './code-examples-animations';
import { initColorLoomAnimations } from './colorloom-animations';
import { initMicroInteractions } from './micro-interactions';
import { initLivingAnimations } from './living-animations';

/**
 * Initialize all GSAP animations on client side
 */
export function initAnimations() {
  console.log('🎬 Initializing GSAP animations...');
  console.log('GSAP available:', typeof gsap !== 'undefined');
  console.log('ScrollTrigger available:', typeof ScrollTrigger !== 'undefined');
  
  try {
    // Test animation to verify GSAP is working
    const testElement = document.querySelector('body');
    if (testElement) {
      gsap.fromTo(testElement, 
        { opacity: 0.99 }, 
        { opacity: 1, duration: 0.1 }
      );
      console.log('✅ GSAP test animation executed');
    }

    // Initialize ScrollSmoother
    const smoother = initScrollSmoother();
    if (smoother) {
      console.log('✅ ScrollSmoother initialized');
    } else {
      console.log('ℹ️ ScrollSmoother skipped (mobile or reduced motion)');
    }
    
    // Create scroll progress indicator
    createScrollProgress();
    console.log('✅ Scroll progress indicator created');

    // Initialize hero animations
    const heroCleanup = initHeroAnimations();
    console.log('✅ Hero animations initialized');

    // Initialize micro-interactions
    initMicroInteractions();
    console.log('✅ Micro-interactions initialized');

    // Initialize living, breathing animations
    setTimeout(() => {
      initLivingAnimations();
      console.log('✅ Living animations initialized');
    }, 150);

    // Initialize color palette animations
    setTimeout(() => {
      initColorPaletteAnimations();
      initCodeExamplesAnimations();
      initColorLoomAnimations();
      console.log('✅ Section animations initialized');
      
      // Refresh ScrollTrigger after all animations are set up
      ScrollTrigger.refresh();
      console.log('✅ ScrollTrigger refreshed');
      console.log('🎉 All animations initialized successfully!');
    }, 300);
  } catch (error) {
    console.error('❌ Error initializing animations:', error);
    console.error('Stack:', error instanceof Error ? error.stack : 'No stack trace');
  }
}

