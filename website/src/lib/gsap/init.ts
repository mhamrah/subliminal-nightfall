import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';
import { ScrollSmoother } from 'gsap/ScrollSmoother';
import { Observer } from 'gsap/Observer';

// Register GSAP plugins
gsap.registerPlugin(ScrollTrigger, ScrollSmoother, Observer);

// Check for reduced motion preference
const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

let scrollSmoother: ScrollSmoother | null = null;

/**
 * Initialize ScrollSmoother for smooth scrolling
 */
export function initScrollSmoother(): ScrollSmoother | null {
  if (prefersReducedMotion) {
    return null;
  }

  // Disable on mobile for better performance
  if (window.innerWidth < 768) {
    return null;
  }

  if (scrollSmoother) {
    return scrollSmoother;
  }

  scrollSmoother = ScrollSmoother.create({
    wrapper: '#smooth-wrapper',
    content: '#smooth-content',
    smooth: 1.5,
    effects: true,
    smoothTouch: 0.1,
    normalizeScroll: true,
    ignoreMobileResize: true,
  });

  return scrollSmoother;
}

/**
 * Cleanup ScrollSmoother
 */
export function cleanupScrollSmoother(): void {
  if (scrollSmoother) {
    scrollSmoother.kill();
    scrollSmoother = null;
  }
}

/**
 * Check if reduced motion is preferred
 */
export function shouldReduceMotion(): boolean {
  return prefersReducedMotion;
}

/**
 * Refresh ScrollTrigger after DOM updates
 */
export function refreshScrollTrigger(): void {
  ScrollTrigger.refresh();
}

export { gsap, ScrollTrigger, ScrollSmoother, Observer };

