import { gsap, ScrollTrigger } from './init';
import { shouldReduceMotion } from './utils';
import { createBreathingEffect, createScrollRearrangement } from './breathing-animations';

/**
 * Initialize code examples scroll-triggered animations
 */
export function initCodeExamplesAnimations(): () => void {
  if (shouldReduceMotion()) return () => {};

  const examplesSection = document.querySelector('#examples');
  if (!examplesSection) return () => {};

  // Animate code tabs with sliding and breathing
  const tabs = examplesSection.querySelectorAll('.code-tab');
  tabs.forEach((tab, index) => {
    gsap.fromTo(
      tab,
      { opacity: 0, x: -50, scale: 0.9, rotation: -3 },
      {
        opacity: 1,
        x: 0,
        scale: 1,
        rotation: 0,
        duration: 0.6,
        ease: 'back.out(1.4)',
        delay: index * 0.08,
        scrollTrigger: {
          trigger: examplesSection,
          start: 'top 80%',
          toggleActions: 'play none none reverse',
        },
      }
    );

    // Add breathing effect
    const breathing = createBreathingEffect(tab as HTMLElement, {
      duration: 2.5 + (index * 0.2),
      scale: 0.008,
      opacity: 0.03,
    });
  });

  // Animate code panels with different directions and breathing
  const panels = examplesSection.querySelectorAll('.code-panel');
  panels.forEach((panel, index) => {
    const directions = ['left', 'right', 'bottom'];
    const direction = directions[index % directions.length];
    
    let fromVars: gsap.TweenVars = { opacity: 0, rotation: -5 };
    if (direction === 'left') {
      fromVars.x = -150;
      fromVars.rotationY = -15;
    } else if (direction === 'right') {
      fromVars.x = 150;
      fromVars.rotationY = 15;
    } else {
      fromVars.y = 80;
      fromVars.rotationX = -10;
    }

    gsap.fromTo(
      panel,
      fromVars,
      {
        opacity: 1,
        x: 0,
        y: 0,
        rotation: 0,
        rotationX: 0,
        rotationY: 0,
        duration: 1,
        ease: 'power3.out',
        scrollTrigger: {
          trigger: panel,
          start: 'top 85%',
          toggleActions: 'play none none reverse',
        },
      }
    );

    // Add breathing effect
    const breathing = createBreathingEffect(panel as HTMLElement, {
      duration: 4 + (index * 0.3),
      scale: 0.01,
      opacity: 0.02,
    });

    // Scroll rearrangement
    ScrollTrigger.create({
      trigger: panel,
      start: 'top 80%',
      end: 'bottom 20%',
      scrub: 1,
      onUpdate: (self) => {
        const progress = self.progress;
        const offset = (progress - 0.5) * 40 * (index % 2 === 0 ? 1 : -1);
        const rotation = (progress - 0.5) * 4;
        gsap.set(panel, {
          x: offset,
          rotationY: rotation,
          scale: 1 + (Math.sin(progress * Math.PI) * 0.015),
        });
      },
    });
  });

  // Enhanced tab switching animation
  const tabButtons = examplesSection.querySelectorAll('.code-tab');
  const codePanels = examplesSection.querySelectorAll('.code-panel');

  tabButtons.forEach((tab) => {
    tab.addEventListener('click', () => {
      const lang = tab.getAttribute('data-lang');
      if (!lang) return;

      // Animate active tab
      tabButtons.forEach((t) => {
        gsap.to(t, {
          duration: 0.3,
          scale: t === tab ? 1.05 : 1,
          ease: 'power2.out',
        });
      });

      // Animate panel switch
      codePanels.forEach((panel) => {
        const isActive = panel.getAttribute('data-lang') === lang;
        if (isActive) {
          gsap.fromTo(
            panel,
            { opacity: 0, scale: 0.95 },
            {
              opacity: 1,
              scale: 1,
              duration: 0.4,
              ease: 'power2.out',
            }
          );
        } else {
          gsap.to(panel, {
            opacity: 0,
            scale: 0.95,
            duration: 0.3,
            ease: 'power2.in',
          });
        }
      });
    });
  });

  // Animate syntax highlighting line by line (simulated)
  const codeBlocks = examplesSection.querySelectorAll('pre code');
  codeBlocks.forEach((block) => {
    ScrollTrigger.create({
      trigger: block,
      start: 'top 90%',
      onEnter: () => {
        const lines = block.querySelectorAll('.token');
        if (lines.length > 0) {
          gsap.fromTo(
            lines,
            { opacity: 0, y: 10 },
            {
              opacity: 1,
              y: 0,
              duration: 0.3,
              stagger: 0.02,
              ease: 'power2.out',
            }
          );
        }
      },
      once: true,
    });
  });

  return () => {};
}

