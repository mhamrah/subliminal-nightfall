import { gsap, ScrollTrigger } from './init';
import { createColorPulse, createRippleEffect, shouldReduceMotion } from './utils';
import { createBreathingEffect, createScrollRearrangement, createColorMorph } from './breathing-animations';

/**
 * Initialize color palette horizontal scroll animations
 */
export function initColorPaletteAnimations(): () => void {
  if (shouldReduceMotion()) return () => {};

  const cleanupFunctions: (() => void)[] = [];

  const paletteSection = document.querySelector('#colors');
  if (!paletteSection) return () => {};

  // Create horizontal scroll container with rearrangement
  const colorGrids = paletteSection.querySelectorAll('.grid');
  if (colorGrids.length === 0) return () => {};

  // Pin the section and create horizontal scroll
  colorGrids.forEach((grid, gridIndex) => {
    const cards = grid.querySelectorAll('.bg-sn-box, [class*="bg-sn-box"]');
    
    if (cards.length > 0) {
      // Animate cards on scroll with sliding entrance
      gsap.fromTo(
        cards,
        { opacity: 0, x: 150, scale: 0.85, rotation: -10 },
        {
          opacity: 1,
          x: 0,
          scale: 1,
          rotation: 0,
          duration: 1,
          ease: 'power3.out',
          stagger: {
            amount: 0.8,
            from: 'random',
          },
          scrollTrigger: {
            trigger: grid,
            start: 'top 75%',
            toggleActions: 'play none none reverse',
          },
        }
      );

      // Add scroll rearrangement to grid
      const rearrangementCleanup = createScrollRearrangement(grid as HTMLElement, {
        stagger: 0.05,
        rotation: 1.5,
        scale: 0.03,
      });
      cleanupFunctions.push(rearrangementCleanup);

      // Add breathing and color morphing to color swatches
      cards.forEach((card, cardIndex) => {
        // Breathing effect on card
        const cardBreathing = createBreathingEffect(card as HTMLElement, {
          duration: 4 + (cardIndex * 0.3),
          scale: 0.01,
          opacity: 0.02,
        });
        cleanupFunctions.push(cardBreathing);

        const swatches = card.querySelectorAll('[data-color]');
        swatches.forEach((swatch, swatchIndex) => {
          const color = swatch.getAttribute('data-color');
          if (color) {
            const colorBox = swatch.querySelector('div[style*="background-color"]') as HTMLElement;
            if (colorBox) {
              // Enhanced pulse with color morphing
              const pulseCleanup = createColorPulse(colorBox, color, {
                duration: 2.5 + (swatchIndex * 0.2),
                intensity: 0.25,
              });
              cleanupFunctions.push(pulseCleanup);

              // Scroll-triggered color slide
              ScrollTrigger.create({
                trigger: colorBox,
                start: 'top 90%',
                end: 'bottom 10%',
                scrub: 1,
                onUpdate: (self) => {
                  const progress = self.progress;
                  const hue = progress * 360;
                  gsap.set(colorBox, {
                    filter: `hue-rotate(${hue * 0.1}deg) saturate(${1 + progress * 0.2})`,
                    scale: 1 + (Math.sin(progress * Math.PI * 2) * 0.05),
                  });
                },
              });
            }
          }
        });
      });
    }
  });

  // Enhanced copy animation with ripple
  const swatches = paletteSection.querySelectorAll('.color-swatch');
  swatches.forEach((swatch) => {
    const originalClick = swatch.getAttribute('onclick') || '';
    swatch.addEventListener('click', (e) => {
      const color = swatch.getAttribute('data-color');
      if (color) {
        createRippleEffect(swatch as HTMLElement, color, {
          duration: 0.6,
          scale: 3,
        });
      }
    });
  });

  // Add hover tilt effect to color cards
  const colorCards = paletteSection.querySelectorAll('.bg-sn-box');
  colorCards.forEach((card) => {
    let isHovering = false;
    
    card.addEventListener('mouseenter', () => {
      if (shouldReduceMotion()) return;
      isHovering = true;
      gsap.to(card, {
        duration: 0.3,
        scale: 1.02,
        y: -5,
        rotationY: 2,
        boxShadow: '0 10px 30px rgba(95, 179, 179, 0.2)',
        ease: 'power2.out',
      });
    });

    card.addEventListener('mouseleave', () => {
      if (shouldReduceMotion()) return;
      isHovering = false;
      gsap.to(card, {
        duration: 0.3,
        scale: 1,
        y: 0,
        rotationY: 0,
        boxShadow: '0 0 0 rgba(95, 179, 179, 0)',
        ease: 'power2.out',
      });
    });
  });

  return () => {
    cleanupFunctions.forEach(cleanup => cleanup());
  };
}

