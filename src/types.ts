export type Locale = 'zh' | 'en';
export type View = 'home' | 'supplement' | 'people' | 'person' | 'stories' | 'story' | 'plan';

export interface Supplement {
  id: string;
  nameZh: string;
  nameEn: string;
  category: string;
  tier: string;
  summary: string;
  filePath?: string;
}

export interface Person {
  id: string;
  name: string;
  nameZh?: string;
  summary: string;
  filePath?: string;
  accent: string;
}

export interface Story {
  id: string;
  title: string;
  titleEn?: string;
  summary: string;
  summaryEn?: string;
  filePath?: string;
  accent: string;
}

export interface LibrarySnapshot {
  root: string;
  connected: boolean;
  supplements: Supplement[];
  people: Person[];
  stories: Story[];
  noteCount: number;
}

export interface ModelConfig {
  provider: 'openai' | 'deepseek' | 'openrouter' | 'custom';
  baseUrl: string;
  model: string;
  apiKey: string;
}

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'tool_call';
  content: string;
  createdAt: number;
  toolName?: string;
  toolArgs?: string;
  toolStatus?: 'running' | 'done' | 'failed';
  toolOutput?: string;
}

export interface ChatRequest {
  apiKey: string;
  baseUrl: string;
  model: string;
  question: string;
  locale: Locale;
  knowledgeRoot: string;
  contextPaths: string[];
  history: Array<{ role: 'user' | 'assistant'; content: string }>;
}

export interface CaptureRequest {
  knowledgeRoot: string;
  title: string;
  content: string;
  sourceUrl?: string;
  locale: Locale;
}

export interface PrepareCaptureRequest {
  apiKey: string;
  baseUrl: string;
  model: string;
  input: string;
  locale: Locale;
}

export interface CaptureDraft {
  title: string;
  content: string;
  sourceUrl?: string;
}

// ── Agent events (from Rust backend via Tauri) ──

export type AgentEvent =
  | { type: 'text_delta'; text: string }
  | { type: 'tool_call_start'; id: string; name: string }
  | { type: 'tool_call_args'; id: string; args: string }
  | { type: 'tool_result'; id: string; output: string; success: boolean }
  | { type: 'done' }
  | { type: 'error'; message: string };

export interface ToolCallMessage {
  id: string;
  name: string;
  args: string;
  status: 'running' | 'done' | 'failed';
  output?: string;
}

export interface AgentRequest {
  apiKey: string;
  baseUrl: string;
  model: string;
  message: string;
  locale: Locale;
  knowledgeRoot: string;
  contextPaths: string[];
  history: Array<{ role: 'user' | 'assistant'; content: string }>;
}
