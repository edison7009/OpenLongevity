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
  role: 'user' | 'assistant';
  content: string;
  createdAt: number;
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
