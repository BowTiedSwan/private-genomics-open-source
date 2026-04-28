export default function BrandHelix() {
  return (
    <div className="brand-logo" aria-hidden="true">
      <svg viewBox="0 0 64 64" role="img" focusable="false">
        <defs>
          <radialGradient id="brand-shell" cx="30%" cy="25%" r="90%">
            <stop offset="0%" stopColor="rgba(127, 179, 255, 0.28)" />
            <stop offset="55%" stopColor="rgba(120, 103, 255, 0.2)" />
            <stop offset="100%" stopColor="rgba(24, 28, 39, 0.95)" />
          </radialGradient>
          <linearGradient id="helix-a" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stopColor="rgba(127, 179, 255, 0.95)" />
            <stop offset="100%" stopColor="rgba(138, 94, 255, 0.95)" />
          </linearGradient>
          <linearGradient id="helix-b" x1="100%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" stopColor="rgba(197, 139, 255, 0.95)" />
            <stop offset="100%" stopColor="rgba(64, 223, 255, 0.95)" />
          </linearGradient>
          <filter id="helix-glow" x="-30%" y="-30%" width="160%" height="160%">
            <feGaussianBlur stdDeviation="1.6" result="blur" />
            <feMerge>
              <feMergeNode in="blur" />
              <feMergeNode in="SourceGraphic" />
            </feMerge>
          </filter>
        </defs>

        <circle cx="32" cy="32" r="30" className="brand-logo-bg" />
        <g filter="url(#helix-glow)">
          <path
            className="brand-helix brand-helix-a"
            d="M20 10c10 8 14 16 14 22s-4 14-14 22"
            fill="none"
            stroke="url(#helix-a)"
            strokeWidth="4.5"
            strokeLinecap="round"
          />
          <path
            className="brand-helix brand-helix-b"
            d="M44 10C34 18 30 26 30 32s4 14 14 22"
            fill="none"
            stroke="url(#helix-b)"
            strokeWidth="4.5"
            strokeLinecap="round"
          />

          <path d="M23 16h18" className="brand-rung" />
          <path d="M20 24h24" className="brand-rung" />
          <path d="M18 32h28" className="brand-rung" />
          <path d="M20 40h24" className="brand-rung" />
          <path d="M23 48h18" className="brand-rung" />
        </g>
      </svg>
    </div>
  );
}
