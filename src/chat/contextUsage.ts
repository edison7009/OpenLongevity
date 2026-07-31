import type { ChatMessage } from '../types';

export const AGENT_CONTEXT_MAX_BYTES = 1_000_000;

const encoder = new TextEncoder();

export function estimateContextBytes(messages: readonly ChatMessage[]): number {
  return messages.reduce((total, message) => {
    if (message.role === 'tool_call') {
      return (
        total +
        130 +
        encoder.encode(message.toolArgs || '').length +
        encoder.encode(message.toolOutput || '').length
      );
    }

    if (message.role === 'memory_suggestion') {
      return total;
    }

    return total + 50 + encoder.encode(message.content).length;
  }, 0);
}

export function formatContextUsage(bytes: number, maxBytes: number): string {
  const percent = Math.min(100, Math.round((bytes / maxBytes) * 100));
  return `${percent}%`;
}
