import { gsap, ScrollTrigger } from './init';
import { shouldReduceMotion } from './utils';
import { createBreathingEffect, createParallaxLayers, createScrollRearrangement, createSlidingGradient } from './breathing-animations';

/**
 * Initialize living, breathing animations throughout the site
 */
export function initLivingAnimations(): () => void {
  if (shouldReduceMotion()) return () => {};

  const cleanupFunctions: (() => void)[] = [];

  // Add breathing effect to section headers
  const sectionHeaders = document.querySelectorAll('section h2, section h3');
  sectionHeaders.forEach((header, index) => {
    const breathing = createBreathingEffect(header as HTMLElement, {
      duration: 4 + (index * 0.3),
      scale: 0.01,
      opacity: 0.05,
    });
    cleanupFunctions.push(breathing);

    // Color sliding on scroll with smooth transitions
    const colors = ['#5fb3b3', '#6699cc', '#c4a7e7', '#f1a5ab'];
    let currentColorIndex = 0;
    
    ScrollTrigger.create({
      trigger: header,
      start: 'top 80%',
      end: 'bottom 20%',
      scrub: 1,
      onUpdate: (self) => {
        const progress = self.progress;
        const newColorIndex = Math.floor(progress * (colors.length - 1));
        
        if (newColorIndex !== currentColorIndex) {
          currentColorIndex = newColorIndex;
          gsap.to(header, {
            duration: 0.5,
            color: colors[currentColorIndex],
            ease: 'power2.out',
          });
        }
      },
    });
  });

  // Parallax layers for background elements
  const parallaxElements = [
    document.querySelector('body'),
    document.querySelector('#smooth-content'),
  ].filter(Boolean) as HTMLElement[];

  if (parallaxElements.length > 0) {
    const parallaxCleanup = createParallaxLayers(
      parallaxElements,
      [0.3, 0.5],
      { direction: 'vertical' }
    );
    cleanupFunctions.push(parallaxCleanup);
  }

  // Add breathing to code panels
  const codePanels = document.querySelectorAll('.code-panel');
  codePanels.forEach((panel, index) => {
    const breathing = createBreathingEffect(panel as HTMLElement, {
      duration: 5 + (index * 0.4),
      scale: 0.008,
      opacity: 0.02,
    });
    cleanupFunctions.push(breathing);

    // Scroll rearrangement
    ScrollTrigger.create({
      trigger: panel,
      start: 'top 85%',
      end: 'bottom 15%',
      scrub: 1,
      onUpdate: (self) => {
        const progress = self.progress;
        gsap.set(panel, {
          x: (progress - 0.5) * 30 * (index % 2 === 0 ? 1 : -1),
          rotationY: (progress - 0.5) * 2,
          scale: 1 + (Math.sin(progress * Math.PI) * 0.01),
        });
      },
    });
  });

  // Add color sliding to buttons
  const buttons = document.querySelectorAll('a[class*="bg-"], button[class*="bg-"]');
  buttons.forEach((button) => {
    const bgColor = window.getComputedStyle(button).backgroundColor;
    if (bgColor && bgColor !== 'rgba(0, 0, 0, 0)') {
      ScrollTrigger.create({
        trigger: button,
        start: 'top 90%',
        end: 'bottom 10%',
        scrub: 1,
        onUpdate: (self) => {
          const progress = self.progress;
          const colors = ['#5fb3b3', '#6699cc', '#c4a7e7'];
          const colorIndex = Math.floor(progress * colors.length) % colors.length;
          gsap.set(button, {
            backgroundColor: colors[colorIndex],
            boxShadow: `0 0 ${20 + progress * 30}px ${colors[colorIndex]}40`,
          });
        },
      });
    }
  });

  // Add breathing to installation cards
  const installCards = document.querySelectorAll('.platform-content');
  installCards.forEach((card, index) => {
    const breathing = createBreathingEffect(card as HTMLElement, {
      duration: 3.5 + (index * 0.2),
      scale: 0.012,
      opacity: 0.03,
    });
    cleanupFunctions.push(breathing);
  });

  // Rearrangement for color palette grids
  const colorGrids = document.querySelectorAll('#colors .grid');
  colorGrids.forEach((grid) => {
    const rearrangement = createScrollRearrangement(grid as HTMLElement, {
      stagger: 0.08,
      rotation: 2,
      scale: 0.04,
    });
    cleanupFunctions.push(rearrangement);
  });

  // Add sliding gradient to CTA sections
  const ctaSections = document.querySelectorAll('section[class*="gradient"]');
  ctaSections.forEach((section) => {
    const colors = ['#31748f', '#5fb3b3', '#6699cc', '#c4a7e7'];
    const gradient = createSlidingGradient(section as HTMLElement, colors, {
      duration: 15,
      angle: 45,
    });
    cleanupFunctions.push(gradient);
  });

  return () => {
    cleanupFunctions.forEach(cleanup => cleanup());
  };
}

