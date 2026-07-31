import { useId } from 'react';

interface ContextRingProps {
  bytes: number;
  maxBytes: number;
  label: string;
  description: string;
  compactedLabel: string;
}

export function ContextRing({
  bytes,
  maxBytes,
  label,
  description,
  compactedLabel,
}: ContextRingProps) {
  const tooltipId = useId();
  const ratio = maxBytes > 0 ? bytes / maxBytes : 0;
  const clamped = Math.min(1, Math.max(0, ratio));
  const radius = 14;
  const circumference = 2 * Math.PI * radius;
  const dashOffset = circumference * (1 - clamped);
  const percent = Math.min(100, Math.round(clamped * 100));
  const tone = clamped < 0.7 ? 'ok' : clamped < 0.9 ? 'warn' : 'full';
  const compacted = bytes >= maxBytes;

  return (
    <span className="context-meter">
      <span
        className={`context-ring ${tone}`}
        aria-describedby={tooltipId}
        aria-label={label}
        role="meter"
        tabIndex={0}
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={percent}
      >
        <svg aria-hidden="true" viewBox="0 0 32 32" className="context-ring-svg">
          <circle cx="16" cy="16" r={radius} fill="none" strokeWidth="1.5" className="context-ring-track" />
          <circle
            cx="16"
            cy="16"
            r={radius}
            fill="none"
            strokeWidth="1.5"
            strokeLinecap="round"
            strokeDasharray={circumference}
            strokeDashoffset={dashOffset}
            className="context-ring-progress"
          />
        </svg>
        <span className="context-ring-label">{percent}%</span>
      </span>
      <span className="context-tooltip" id={tooltipId} role="tooltip">
        <strong>{label}</strong>
        <span>{description}</span>
        {compacted && <small>{compactedLabel}</small>}
      </span>
    </span>
  );
}
