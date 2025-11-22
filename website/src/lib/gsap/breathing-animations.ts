import { gsap, ScrollTrigger, shouldReduceMotion } from './init';

/**
 * Create breathing/pulsing animations for elements
 */
export function createBreathingEffect(
  element: HTMLElement | string,
  options: { duration?: number; scale?: number; opacity?: number } = {}
): () => void {
  if (shouldReduceMotion()) return () => {};

  const el = typeof element === 'string' ? document.querySelector(element) as HTMLElement : element;
  if (!el) return () => {};

  const { duration = 3, scale = 0.02, opacity = 0.05 } = options;

  const tl = gsap.timeline({ repeat: -1, yoyo: true, ease: 'sine.inOut' });
  
  tl.to(el, {
    duration,
    scale: 1 + scale,
    opacity: `+=${opacity}`,
    ease: 'sine.inOut',
  });

  return () => tl.kill();
}

/**
 * Create color sliding/morphing effect
 */
export function createColorMorph(
  element: HTMLElement | string,
  colors: string[],
  options: { duration?: number; ease?: string } = {}
): () => void {
  if (shouldReduceMotion()) return () => {};

  const el = typeof element === 'string' ? document.querySelector(element) as HTMLElement : element;
  if (!el) return () => {};

  const { duration = 8, ease = 'sine.inOut' } = options;

  const tl = gsap.timeline({ repeat: -1, yoyo: true, ease });
  
  colors.forEach((color, index) => {
    tl.to(el, {
      duration: duration / colors.length,
      backgroundColor: color,
      ease: 'sine.inOut',
    }, index * (duration / colors.length));
  });

  return () => tl.kill();
}

/**
 * Create sliding color gradient animation
 */
export function createSlidingGradient(
  element: HTMLElement | string,
  colors: string[],
  options: { duration?: number; angle?: number } = {}
): () => void {
  if (shouldReduceMotion()) return () => {};

  const el = typeof element === 'string' ? document.querySelector(element) as HTMLElement : element;
  if (!el) return () => {};

  const { duration = 10, angle = 45 } = options;
  
  // Create gradient string
  const gradientStops = colors.map((color, i) => 
    `${color} ${(i / (colors.length - 1)) * 100}%`
  ).join(', ');
  
  const gradient = `linear-gradient(${angle}deg, ${gradientStops})`;
  
  gsap.set(el, {
    backgroundImage: gradient,
    backgroundSize: '200% 200%',
    backgroundPosition: '0% 50%',
  });

  const tl = gsap.timeline({ repeat: -1, yoyo: true, ease: 'sine.inOut' });
  
  tl.to(el, {
    duration,
    backgroundPosition: '100% 50%',
    ease: 'sine.inOut',
  });

  return () => tl.kill();
}

/**
 * Create parallax effect with multiple layers
 */
export function createParallaxLayers(
  elements: (HTMLElement | string)[],
  speeds: number[],
  options: { direction?: 'vertical' | 'horizontal' } = {}
): () => void {
  if (shouldReduceMotion()) return () => {};

  const { direction = 'vertical' } = options;
  const cleanupFunctions: (() => void)[] = [];

  elements.forEach((element, index) => {
    const el = typeof element === 'string' ? document.querySelector(element) as HTMLElement : element;
    if (!el) return;

    const speed = speeds[index] || 0.5;
    const prop = direction === 'vertical' ? 'y' : 'x';

    ScrollTrigger.create({
      trigger: el,
      start: 'top bottom',
      end: 'bottom top',
      scrub: true,
      onUpdate: (self) => {
        const progress = self.progress;
        const offset = (progress - 0.5) * speed * 100;
        gsap.set(el, { [prop]: offset });
      },
    });
  });

  return () => {
    cleanupFunctions.forEach(cleanup => cleanup());
  };
}

/**
 * Create element rearrangement on scroll
 */
export function createScrollRearrangement(
  container: HTMLElement | string,
  options: { stagger?: number; rotation?: number; scale?: number } = {}
): () => void {
  if (shouldReduceMotion()) return () => {};

  const cont = typeof container === 'string' ? document.querySelector(container) as HTMLElement : container;
  if (!cont) return () => {};

  const { stagger = 0.1, rotation = 2, scale = 0.05 } = options;
  const children = Array.from(cont.children) as HTMLElement[];

  children.forEach((child, index) => {
    ScrollTrigger.create({
      trigger: child,
      start: 'top 80%',
      end: 'bottom 20%',
      scrub: 1,
      onUpdate: (self) => {
        const progress = self.progress;
        const delay = index * stagger;
        const adjustedProgress = Math.max(0, Math.min(1, progress - delay));
        
        gsap.to(child, {
          duration: 0.1,
          rotation: adjustedProgress * rotation * (index % 2 === 0 ? 1 : -1),
          scale: 1 + (adjustedProgress * scale),
          x: adjustedProgress * 10 * (index % 2 === 0 ? 1 : -1),
          ease: 'none',
        });
      },
    });
  });

  return () => {};
}

