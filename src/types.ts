export type Locale = 'zh' | 'en';
export type View = 'home' | 'ai' | 'supplement' | 'people' | 'person' | 'stories' | 'story' | 'plan';

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

export type ModelProvider = 'openai' | 'anthropic';

export interface ProviderConfig {
  baseUrl: string;
  model: string;
  apiKey: string;
}

export interface ModelConfig extends ProviderConfig {
  provider: ModelProvider;
}

export interface ModelSettings {
  activeProvider: ModelProvider;
  providers: Record<ModelProvider, ProviderConfig>;
}

export interface MemorySuggestion {
  id: string;
  kind: 'goal' | 'preference' | 'constraint' | 'profile' | 'correction' | 'health_context';
  content: string;
  sourceConversationId: string;
}

export interface MemoryItem extends MemorySuggestion {
  createdAt: number;
  updatedAt: number;
}

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'tool_call' | 'memory_suggestion';
  content: string;
  createdAt: number;
  toolName?: string;
  toolArgs?: string;
  toolStatus?: 'running' | 'done' | 'failed';
  toolOutput?: string;
  memorySuggestion?: MemorySuggestion;
  memoryStatus?: 'pending' | 'saved' | 'dismissed';
}

export interface ConversationSummary {
  id: string;
  title: string;
  createdAt: number;
  updatedAt: number;
  messageCount: number;
  estimatedContextBytes: number;
}

export interface ConversationRecord {
  id: string;
  title: string;
  createdAt: number;
  updatedAt: number;
  uiMessages: ChatMessage[];
  llmMessages: unknown[];
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
  | { type: 'text_delta'; conversationId?: string; text: string }
  | { type: 'tool_call_start'; conversationId?: string; id: string; name: string }
  | { type: 'tool_call_args'; conversationId?: string; id: string; args: string }
  | { type: 'tool_result'; conversationId?: string; id: string; output: string; success: boolean }
  | { type: 'memory_suggestion'; conversationId?: string; suggestion: MemorySuggestion }
  | { type: 'done'; conversationId?: string }
  | { type: 'error'; conversationId?: string; message: string }
  | { type: 'state'; conversationId?: string; state: string };

export interface ToolCallMessage {
  id: string;
  name: string;
  args: string;
  status: 'running' | 'done' | 'failed';
  output?: string;
}

export interface AgentRequest {
  conversationId: string;
  apiKey: string;
  baseUrl: string;
  model: string;
  provider?: string;
  message: string;
  locale: Locale;
  knowledgeRoot: string;
  contextPaths: string[];
  history: Array<{ role: 'user' | 'assistant'; content: string }>;
}
