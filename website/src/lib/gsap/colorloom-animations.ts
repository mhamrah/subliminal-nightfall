import { gsap, ScrollTrigger } from './init';
import { shouldReduceMotion } from './utils';

/**
 * Initialize ColorLoom showcase animations
 */
export function initColorLoomAnimations(): () => void {
  if (shouldReduceMotion()) return () => {};

  const colorloomSection = document.querySelector('#colorloom');
  if (!colorloomSection) return () => {};

  // Pin the section for scroll animation
  const pinnedContainer = colorloomSection.querySelector('.colorloom-pinned-container');
  if (pinnedContainer) {
    ScrollTrigger.create({
      trigger: pinnedContainer,
      start: 'top top',
      end: '+=3000',
      pin: true,
      pinSpacing: true,
    });
  }

  // Animate config preview on scroll
  const configPreview = colorloomSection.querySelector('#config-preview');
  if (configPreview) {
    gsap.fromTo(
      configPreview,
      { opacity: 0, scale: 0.95 },
      {
        opacity: 1,
        scale: 1,
        duration: 0.8,
        ease: 'power3.out',
        scrollTrigger: {
          trigger: configPreview,
          start: 'top 80%',
          toggleActions: 'play none none reverse',
        },
      }
    );
  }

  return () => {};
}

