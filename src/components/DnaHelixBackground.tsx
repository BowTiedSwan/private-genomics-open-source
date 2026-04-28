import { useEffect, useRef } from "react";
import { animate } from "animejs";

interface Point3D {
  x: number;
  y: number;
  z: number;
}

interface ProjectedPoint {
  x: number;
  y: number;
  scale: number;
  z: number;
}

export default function DnaHelixBackground() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    let width = 0;
    let height = 0;
    let animationFrameId: number;
    const animState = { rotation: 0, pulse: 0 };
    const prefersReducedMotion = window.matchMedia(
      "(prefers-reduced-motion: reduce)"
    ).matches;

    const resize = () => {
      const parent = canvas.parentElement;
      if (!parent) return;
      width = parent.clientWidth;
      height = parent.clientHeight;
      const dpr = window.devicePixelRatio || 1;
      canvas.width = width * dpr;
      canvas.height = height * dpr;
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    };

    const project = (p: Point3D, rotY: number): ProjectedPoint => {
      const cosR = Math.cos(rotY);
      const sinR = Math.sin(rotY);
      const x = p.x * cosR + p.z * sinR;
      const z = -p.x * sinR + p.z * cosR;
      const fov = 300;
      const scale = fov / (fov + z);
      return {
        x: x * scale + width / 2,
        y: p.y * scale + height / 2,
        scale,
        z,
      };
    };

    const draw = () => {
      ctx.clearRect(0, 0, width, height);
      const strands: ProjectedPoint[][] = [[], []];
      const pairs = 48;
      const radius = 55;
      const pitch = 12;
      const rotY = animState.rotation;

      for (let i = 0; i < pairs; i++) {
        const t = i * 0.35;
        const strand1: Point3D = {
          x: radius * Math.cos(t),
          y: (i - pairs / 2) * pitch,
          z: radius * Math.sin(t),
        };
        const strand2: Point3D = {
          x: radius * Math.cos(t + Math.PI),
          y: (i - pairs / 2) * pitch,
          z: radius * Math.sin(t + Math.PI),
        };
        strands[0].push(project(strand1, rotY));
        strands[1].push(project(strand2, rotY));
      }

      // Draw strand connections
      ctx.lineWidth = 0.8;
      for (let s = 0; s < 2; s++) {
        for (let i = 0; i < pairs - 1; i++) {
          const p1 = strands[s][i];
          const p2 = strands[s][i + 1];
          ctx.strokeStyle = `rgba(79, 172, 254, ${0.18 * p1.scale})`;
          ctx.beginPath();
          ctx.moveTo(p1.x, p1.y);
          ctx.lineTo(p2.x, p2.y);
          ctx.stroke();
        }
      }

      // Draw rungs and polygonal cross-connections
      for (let i = 0; i < pairs; i++) {
        const p1 = strands[0][i];
        const p2 = strands[1][i];
        const avgScale = (p1.scale + p2.scale) / 2;
        const alpha = 0.22 * avgScale;

        ctx.strokeStyle = `rgba(0, 210, 255, ${alpha})`;
        ctx.beginPath();
        ctx.moveTo(p1.x, p1.y);
        ctx.lineTo(p2.x, p2.y);
        ctx.stroke();

        if (i < pairs - 1) {
          const p1Next = strands[0][i + 1];
          const p2Next = strands[1][i + 1];

          ctx.strokeStyle = `rgba(58, 106, 176, ${0.1 * avgScale})`;
          ctx.beginPath();
          ctx.moveTo(p1.x, p1.y);
          ctx.lineTo(p2Next.x, p2Next.y);
          ctx.stroke();

          ctx.beginPath();
          ctx.moveTo(p2.x, p2.y);
          ctx.lineTo(p1Next.x, p1Next.y);
          ctx.stroke();
        }
      }

      // Draw nodes with glow
      const pulseFactor = 1 + 0.25 * Math.sin(animState.pulse * Math.PI * 2);
      for (let s = 0; s < 2; s++) {
        for (let i = 0; i < pairs; i++) {
          const p = strands[s][i];
          const baseSize = s === 0 ? 2.2 : 1.8;
          const size = baseSize * p.scale * pulseFactor;
          const glowRadius = size * 5;

          const glow = ctx.createRadialGradient(
            p.x,
            p.y,
            0,
            p.x,
            p.y,
            glowRadius
          );
          glow.addColorStop(0, `rgba(127, 179, 255, ${0.9 * p.scale})`);
          glow.addColorStop(0.4, `rgba(0, 210, 255, ${0.35 * p.scale})`);
          glow.addColorStop(1, "rgba(0, 210, 255, 0)");

          ctx.fillStyle = glow;
          ctx.beginPath();
          ctx.arc(p.x, p.y, glowRadius, 0, Math.PI * 2);
          ctx.fill();

          ctx.fillStyle = `rgba(255, 255, 255, ${0.95 * p.scale})`;
          ctx.beginPath();
          ctx.arc(p.x, p.y, size, 0, Math.PI * 2);
          ctx.fill();
        }
      }
    };

    const loop = () => {
      draw();
      animationFrameId = requestAnimationFrame(loop);
    };

    resize();
    window.addEventListener("resize", resize);

    if (prefersReducedMotion) {
      // Draw one static frame and stop
      draw();
      return () => {
        window.removeEventListener("resize", resize);
      };
    }

    const rotationAnim = animate(animState, {
      rotation: Math.PI * 2,
      duration: 40000,
      loop: true,
      ease: "linear",
    });

    const pulseAnim = animate(animState, {
      pulse: 2,
      duration: 3500,
      loop: true,
      alternate: true,
      ease: "easeInOutSine",
    });

    loop();

    return () => {
      cancelAnimationFrame(animationFrameId);
      rotationAnim.pause();
      pulseAnim.pause();
      window.removeEventListener("resize", resize);
    };
  }, []);

  return (
    <canvas
      ref={canvasRef}
      aria-hidden="true"
      style={{
        width: "100%",
        height: "100%",
        display: "block",
      }}
    />
  );
}
