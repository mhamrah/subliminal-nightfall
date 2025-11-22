import { gsap, ScrollTrigger, shouldReduceMotion } from './init';

/**
 * Create a scroll progress indicator at the top of the page
 */
export function createScrollProgress(): () => void {
  if (shouldReduceMotion()) return () => {};

  // Create progress bar element
  const progressBar = document.createElement('div');
  progressBar.id = 'scroll-progress';
  progressBar.style.cssText = `
    position: fixed;
    top: 0;
    left: 0;
    width: 0%;
    height: 3px;
    background: linear-gradient(90deg, #5fb3b3, #6699cc, #c4a7e7);
    z-index: 9999;
    transition: opacity 0.3s;
  `;
  document.body.appendChild(progressBar);

  ScrollTrigger.create({
    trigger: 'body',
    start: 'top top',
    end: 'bottom bottom',
    onUpdate: (self) => {
      const progress = self.progress;
      gsap.to(progressBar, {
        width: `${progress * 100}%`,
        duration: 0.1,
        ease: 'none',
      });
    },
  });

  return () => {
    if (progressBar.parentNode) {
      document.body.removeChild(progressBar);
    }
  };
}

