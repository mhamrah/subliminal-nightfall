import { gsap, ScrollTrigger } from './init';
import { createMagneticEffect, createFloatingAnimation, shouldReduceMotion } from './utils';
import { createBreathingEffect, createSlidingGradient } from './breathing-animations';

/**
 * Initialize hero section animations
 */
export function initHeroAnimations(): () => void {
  if (shouldReduceMotion()) {
    console.log('⚠️ Reduced motion preference detected, skipping animations');
    return () => {};
  }

  const cleanupFunctions: (() => void)[] = [];
  
  const homeSection = document.querySelector('#home');
  if (!homeSection) {
    console.warn('⚠️ Hero section (#home) not found');
    return () => {};
  }

  // Morphing gradient background with sliding colors
  const gradientBox = document.querySelector('#home .hero-logo') || 
                      document.querySelector('#home .inline-flex.items-center.justify-center.w-24');
  if (gradientBox) {
    console.log('✅ Found hero logo, applying animations');
    
    // Simple visible test animation first
    gsap.fromTo(gradientBox, 
      { scale: 0.8, opacity: 0.5, rotation: -10 },
      { 
        scale: 1, 
        opacity: 1, 
        rotation: 0,
        duration: 1,
        ease: 'back.out(1.7)',
        delay: 0.2
      }
    );
    
    const colors = ['#5fb3b3', '#6699cc', '#c4a7e7', '#f1a5ab'];
    const gradientCleanup = createSlidingGradient(gradientBox as HTMLElement, colors, {
      duration: 12,
      angle: 135,
    });
    cleanupFunctions.push(gradientCleanup);
    
    // Add breathing effect
    const breathingCleanup = createBreathingEffect(gradientBox as HTMLElement, {
      duration: 4,
      scale: 0.03,
      opacity: 0.1,
    });
    cleanupFunctions.push(breathingCleanup);
  } else {
    console.warn('⚠️ Hero logo not found');
  }

  // Floating logo animation
  const logo = document.querySelector('#home .inline-flex.items-center.justify-center.w-24');
  if (logo) {
    const floatCleanup = createFloatingAnimation(logo as HTMLElement, {
      duration: 4,
      y: 15,
      rotation: 3,
    });
    cleanupFunctions.push(floatCleanup);
  }

  // Staggered text reveals (simplified on mobile)
  const title = document.querySelector('#home h1');
  if (title) {
    const isMobile = window.innerWidth < 768;
    
    if (isMobile) {
      // Simple fade-in on mobile
      gsap.fromTo(
        title,
        { opacity: 0, y: 20 },
        {
          opacity: 1,
          y: 0,
          duration: 0.8,
          ease: 'power3.out',
          delay: 0.3,
        }
      );
    } else {
      const titleText = title.textContent || '';
      const chars = titleText.split('').filter(char => char.trim() !== '');
      title.innerHTML = chars.map((char, i) => 
        `<span class="hero-char" style="display: inline-block; opacity: 0;">${char === ' ' ? '&nbsp;' : char}</span>`
      ).join('');

      const charElements = title.querySelectorAll('.hero-char');
      gsap.fromTo(
        charElements,
        { opacity: 0, y: 50, rotationX: -90 },
        {
          opacity: 1,
          y: 0,
          rotationX: 0,
          duration: 0.8,
          ease: 'back.out(1.7)',
          stagger: 0.03,
          delay: 0.3,
        }
      );
    }
  }

  // Subtitle animation
  const subtitle = document.querySelector('#home p.text-xl');
  if (subtitle) {
    gsap.fromTo(
      subtitle,
      { opacity: 0, y: 30 },
      {
        opacity: 1,
        y: 0,
        duration: 1,
        ease: 'power3.out',
        delay: 1.2,
      }
    );

    // Animate colored spans
    const coloredSpans = subtitle.querySelectorAll('span');
    coloredSpans.forEach((span, i) => {
      gsap.fromTo(
        span,
        { opacity: 0, scale: 0.8 },
        {
          opacity: 1,
          scale: 1,
          duration: 0.5,
          ease: 'back.out(1.7)',
          delay: 1.5 + i * 0.1,
        }
      );
    });
  }

  // Feature cards stagger animation with breathing and rearrangement
  const featureCards = document.querySelectorAll('#home .bg-sn-box');
  if (featureCards.length > 0) {
    gsap.fromTo(
      featureCards,
      { opacity: 0, y: 50, scale: 0.9, rotation: -5 },
      {
        opacity: 1,
        y: 0,
        scale: 1,
        rotation: 0,
        duration: 0.8,
        ease: 'power3.out',
        stagger: 0.15,
        scrollTrigger: {
          trigger: featureCards[0],
          start: 'top 85%',
          toggleActions: 'play none none reverse',
        },
      }
    );

    // Add breathing effect to each card
    featureCards.forEach((card, index) => {
      const breathingCleanup = createBreathingEffect(card as HTMLElement, {
        duration: 3 + (index * 0.5),
        scale: 0.015,
        opacity: 0.03,
      });
      cleanupFunctions.push(breathingCleanup);

      // Scroll-triggered rearrangement
      ScrollTrigger.create({
        trigger: card,
        start: 'top 80%',
        end: 'bottom 20%',
        scrub: 1,
        onUpdate: (self) => {
          const progress = self.progress;
          const offset = (progress - 0.5) * 20 * (index % 2 === 0 ? 1 : -1);
          const rotation = (progress - 0.5) * 3;
          gsap.set(card, {
            x: offset,
            rotation: rotation,
            scale: 1 + (Math.sin(progress * Math.PI) * 0.02),
          });
        },
      });
    });
  }

  // Magnetic button effects
  const buttons = document.querySelectorAll('#home a[href], #home button');
  buttons.forEach((button) => {
    const cleanup = createMagneticEffect(button as HTMLElement, 0.2);
    cleanupFunctions.push(cleanup);
  });

  // CTA buttons animation
  const ctaButtons = document.querySelectorAll('#home .inline-flex.items-center');
  if (ctaButtons.length > 0) {
    gsap.fromTo(
      ctaButtons,
      { opacity: 0, scale: 0.8, y: 20 },
      {
        opacity: 1,
        scale: 1,
        y: 0,
        duration: 0.6,
        ease: 'back.out(1.7)',
        stagger: 0.1,
        delay: 1.8,
      }
    );
  }

  return () => {
    cleanupFunctions.forEach(cleanup => cleanup());
  };
}

