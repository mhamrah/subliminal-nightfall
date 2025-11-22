import { gsap, ScrollTrigger } from './init';
import { shouldReduceMotion } from './utils';

/**
 * Initialize micro-interactions throughout the site
 */
export function initMicroInteractions(): () => void {
  if (shouldReduceMotion()) return () => {};

  // Enhanced link animations
  const links = document.querySelectorAll('a[href]');
  links.forEach((link) => {
    link.addEventListener('mouseenter', () => {
      gsap.to(link, {
        duration: 0.3,
        scale: 1.05,
        ease: 'power2.out',
      });
    });

    link.addEventListener('mouseleave', () => {
      gsap.to(link, {
        duration: 0.3,
        scale: 1,
        ease: 'power2.out',
      });
    });
  });

  // Enhanced button hover effects
  const buttons = document.querySelectorAll('button, .btn, [role="button"]');
  buttons.forEach((button) => {
    button.addEventListener('mouseenter', () => {
      gsap.to(button, {
        duration: 0.2,
        scale: 1.05,
        boxShadow: '0 10px 25px rgba(95, 179, 179, 0.3)',
        ease: 'power2.out',
      });
    });

    button.addEventListener('mouseleave', () => {
      gsap.to(button, {
        duration: 0.2,
        scale: 1,
        boxShadow: '0 0 0 rgba(95, 179, 179, 0)',
        ease: 'power2.out',
      });
    });
  });

  // Enhanced toast notification animation
  const toast = document.getElementById('copy-toast');
  if (toast) {
    // Override existing toast animation
    const originalShow = () => {
      gsap.fromTo(
        toast,
        { y: 100, opacity: 0, scale: 0.8 },
        {
          y: 0,
          opacity: 1,
          scale: 1,
          duration: 0.4,
          ease: 'back.out(1.7)',
        }
      );
    };

    const originalHide = () => {
      gsap.to(toast, {
        y: 100,
        opacity: 0,
        scale: 0.8,
        duration: 0.3,
        ease: 'power2.in',
      });
    };

    // Store original functions for use in color palette component
    (toast as any).gsapShow = originalShow;
    (toast as any).gsapHide = originalHide;
  }

  // Card hover effects
  const cards = document.querySelectorAll('.bg-sn-box, [class*="card"]');
  cards.forEach((card) => {
    card.addEventListener('mouseenter', () => {
      gsap.to(card, {
        duration: 0.3,
        y: -5,
        boxShadow: '0 10px 30px rgba(95, 179, 179, 0.15)',
        ease: 'power2.out',
      });
    });

    card.addEventListener('mouseleave', () => {
      gsap.to(card, {
        duration: 0.3,
        y: 0,
        boxShadow: '0 0 0 rgba(95, 179, 179, 0)',
        ease: 'power2.out',
      });
    });
  });

  // Icon hover effects
  const icons = document.querySelectorAll('svg');
  icons.forEach((icon) => {
    icon.addEventListener('mouseenter', () => {
      gsap.to(icon, {
        duration: 0.2,
        scale: 1.1,
        rotation: 5,
        ease: 'power2.out',
      });
    });

    icon.addEventListener('mouseleave', () => {
      gsap.to(icon, {
        duration: 0.2,
        scale: 1,
        rotation: 0,
        ease: 'power2.out',
      });
    });
  });

  // Installation platform cards
  const platformCards = document.querySelectorAll('.platform-content .bg-sn-bg');
  platformCards.forEach((card) => {
    gsap.fromTo(
      card,
      { opacity: 0, y: 20 },
      {
        opacity: 1,
        y: 0,
        duration: 0.6,
        ease: 'power3.out',
        scrollTrigger: {
          trigger: card,
          start: 'top 85%',
          toggleActions: 'play none none reverse',
        },
      }
    );
  });

  return () => {};
}

