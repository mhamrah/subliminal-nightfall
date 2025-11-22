import { gsap, shouldReduceMotion } from './init';

/**
 * Split text into characters/words for animation
 */
export function splitText(element: HTMLElement | string, type: 'chars' | 'words' = 'chars'): string[] {
  const el = typeof element === 'string' ? document.querySelector(element) as HTMLElement : element;
  if (!el) return [];

  const text = el.textContent || '';
  if (type === 'chars') {
    return text.split('').filter(char => char.trim() !== '');
  } else {
    return text.split(' ').filter(word => word.trim() !== '');
  }
}

/**
 * Create a magnetic hover effect for buttons
 */
export function createMagneticEffect(element: HTMLElement | string, strength: number = 0.3): () => void {
  if (shouldReduceMotion()) return () => {};
  
  // Disable on mobile/touch devices
  if ('ontouchstart' in window || window.innerWidth < 768) {
    return () => {};
  }

  const el = typeof element === 'string' ? document.querySelector(element) as HTMLElement : element;
  if (!el) return () => {};

  let x = 0;
  let y = 0;
  let currentX = 0;
  let currentY = 0;
  let rafId: number | null = null;

  const handleMouseMove = (e: MouseEvent) => {
    const rect = el.getBoundingClientRect();
    const centerX = rect.left + rect.width / 2;
    const centerY = rect.top + rect.height / 2;

    x = (e.clientX - centerX) * strength;
    y = (e.clientY - centerY) * strength;
  };

  const handleMouseEnter = () => {
    gsap.to(el, {
      duration: 0.3,
      ease: 'power2.out',
      x: x,
      y: y,
    });
  };

  const handleMouseLeave = () => {
    gsap.to(el, {
      duration: 0.5,
      ease: 'elastic.out(1, 0.3)',
      x: 0,
      y: 0,
    });
    x = 0;
    y = 0;
  };

  const animate = () => {
    if (Math.abs(x - currentX) > 0.1 || Math.abs(y - currentY) > 0.1) {
      currentX += (x - currentX) * 0.1;
      currentY += (y - currentY) * 0.1;
      gsap.set(el, { x: currentX, y: currentY });
      rafId = requestAnimationFrame(animate);
    }
  };

  el.addEventListener('mousemove', handleMouseMove);
  el.addEventListener('mouseenter', () => {
    handleMouseEnter();
    rafId = requestAnimationFrame(animate);
  });
  el.addEventListener('mouseleave', () => {
    handleMouseLeave();
    if (rafId !== null) {
      cancelAnimationFrame(rafId);
      rafId = null;
    }
  });

  return () => {
    el.removeEventListener('mousemove', handleMouseMove);
    el.removeEventListener('mouseenter', handleMouseEnter);
    el.removeEventListener('mouseleave', handleMouseLeave);
    if (rafId !== null) {
      cancelAnimationFrame(rafId);
    }
    gsap.set(el, { x: 0, y: 0 });
  };
}

/**
 * Create a floating animation
 */
export function createFloatingAnimation(
  element: HTMLElement | string,
  options: { duration?: number; y?: number; rotation?: number } = {}
): () => void {
  if (shouldReduceMotion()) return () => {};

  const el = typeof element === 'string' ? document.querySelector(element) as HTMLElement : element;
  if (!el) return () => {};

  const { duration = 3, y = 20, rotation = 5 } = options;

  const tl = gsap.timeline({ repeat: -1, yoyo: true, ease: 'sine.inOut' });
  
  tl.to(el, {
    duration,
    y: y,
    rotation: rotation,
    ease: 'sine.inOut',
  });

  return () => tl.kill();
}

/**
 * Create a pulse animation with color
 */
export function createColorPulse(
  element: HTMLElement | string,
  color: string,
  options: { duration?: number; intensity?: number } = {}
): () => void {
  if (shouldReduceMotion()) return () => {};

  const el = typeof element === 'string' ? document.querySelector(element) as HTMLElement : element;
  if (!el) return () => {};

  const { duration = 2, intensity = 0.3 } = options;

  const tl = gsap.timeline({ repeat: -1, yoyo: true, ease: 'sine.inOut' });
  
  tl.to(el, {
    duration,
    boxShadow: `0 0 ${20 * intensity}px ${color}${Math.round(intensity * 255).toString(16).padStart(2, '0')}`,
    scale: 1 + intensity * 0.1,
    ease: 'sine.inOut',
  });

  return () => tl.kill();
}

/**
 * Animate element on scroll into view
 */
export function animateOnScroll(
  element: HTMLElement | string,
  animation: gsap.TweenVars,
  options: { trigger?: string | HTMLElement; start?: string; end?: string } = {}
): () => void {
  if (shouldReduceMotion()) {
    const el = typeof element === 'string' ? document.querySelector(element) as HTMLElement : element;
    if (el) gsap.set(el, animation);
    return () => {};
  }

  const el = typeof element === 'string' ? document.querySelector(element) as HTMLElement : element;
  if (!el) return () => {};

  const { trigger = el, start = 'top 80%', end = 'bottom 20%' } = options;

  const scrollTrigger = {
    trigger: typeof trigger === 'string' ? document.querySelector(trigger) : trigger,
    start,
    end,
    toggleActions: 'play none none reverse',
  };

  const tl = gsap.timeline({ scrollTrigger });
  tl.fromTo(el, { ...animation, immediateRender: false }, { ...animation, duration: 1, ease: 'power3.out' });

  return () => {
    tl.kill();
  };
}

/**
 * Create a stagger animation for multiple elements
 */
export function createStaggerAnimation(
  elements: NodeListOf<HTMLElement> | HTMLElement[],
  animation: gsap.TweenVars,
  stagger: number = 0.1
): () => void {
  if (shouldReduceMotion()) {
    elements.forEach(el => gsap.set(el, animation));
    return () => {};
  }

  const tl = gsap.timeline();
  tl.fromTo(elements, animation, {
    ...animation,
    duration: 1,
    ease: 'power3.out',
    stagger,
  });

  return () => tl.kill();
}

/**
 * Create a ripple effect animation
 */
export function createRippleEffect(
  element: HTMLElement | string,
  color: string,
  options: { duration?: number; scale?: number } = {}
): () => void {
  if (shouldReduceMotion()) return () => {};

  const el = typeof element === 'string' ? document.querySelector(element) as HTMLElement : element;
  if (!el) return () => {};

  const { duration = 0.6, scale = 2 } = options;

  const ripple = document.createElement('div');
  ripple.style.position = 'absolute';
  ripple.style.width = '20px';
  ripple.style.height = '20px';
  ripple.style.borderRadius = '50%';
  ripple.style.background = color;
  ripple.style.pointerEvents = 'none';
  ripple.style.opacity = '0.6';
  ripple.style.transform = 'translate(-50%, -50%)';
  
  const rect = el.getBoundingClientRect();
  const x = rect.left + rect.width / 2;
  const y = rect.top + rect.height / 2;
  
  ripple.style.left = `${x}px`;
  ripple.style.top = `${y}px`;
  
  document.body.appendChild(ripple);

  gsap.fromTo(
    ripple,
    { scale: 0, opacity: 0.6 },
    {
      scale,
      opacity: 0,
      duration,
      ease: 'power2.out',
      onComplete: () => {
        document.body.removeChild(ripple);
      },
    }
  );

  return () => {
    if (ripple.parentNode) {
      document.body.removeChild(ripple);
    }
  };
}

