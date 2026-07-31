interface ContextRingProps {
  bytes: number;
  maxBytes: number;
  label: string;
}

export function ContextRing({ bytes, maxBytes, label }: ContextRingProps) {
  const ratio = maxBytes > 0 ? bytes / maxBytes : 0;
  const clamped = Math.min(1, Math.max(0, ratio));
  const radius = 9;
  const circumference = 2 * Math.PI * radius;
  const dashOffset = circumference * (1 - clamped);
  const percent = Math.min(100, Math.round(clamped * 100));
  const tone = clamped < 0.7 ? 'ok' : clamped < 0.9 ? 'warn' : 'full';

  return (
    <span
      className={`context-ring ${tone}`}
      aria-label={label}
      role="meter"
      aria-valuemin={0}
      aria-valuemax={100}
      aria-valuenow={percent}
    >
      <svg aria-hidden="true" viewBox="0 0 20 20" className="context-ring-svg">
        <circle cx="10" cy="10" r={radius} fill="none" strokeWidth="1.5" className="context-ring-track" />
        <circle
          cx="10"
          cy="10"
          r={radius}
          fill="none"
          strokeWidth="1.5"
          strokeLinecap="round"
          strokeDasharray={circumference}
          strokeDashoffset={dashOffset}
          className="context-ring-progress"
        />
      </svg>
      <span className="context-ring-label">{percent}</span>
    </span>
  );
}
