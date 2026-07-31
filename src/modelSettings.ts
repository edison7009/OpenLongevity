import type { ModelConfig, ModelProvider, ModelSettings, ProviderConfig } from './types';

const emptyProviderConfig = (): ProviderConfig => ({
  baseUrl: '',
  model: '',
  apiKey: '',
});

export function createEmptyModelSettings(): ModelSettings {
  return {
    activeProvider: 'openai',
    providers: {
      openai: emptyProviderConfig(),
      anthropic: emptyProviderConfig(),
    },
  };
}

export function migrateModelProvider(raw: unknown): ModelProvider {
  if (raw === 'anthropic') return 'anthropic';
  return 'openai';
}

function readProviderConfig(value: unknown): ProviderConfig {
  if (!value || typeof value !== 'object') return emptyProviderConfig();
  const config = value as Record<string, unknown>;
  return {
    baseUrl: typeof config.baseUrl === 'string' ? config.baseUrl : '',
    model: typeof config.model === 'string' ? config.model : '',
    apiKey: typeof config.apiKey === 'string' ? config.apiKey : '',
  };
}

export function normalizeModelSettings(value: unknown): ModelSettings {
  const empty = createEmptyModelSettings();
  if (!value || typeof value !== 'object') return empty;

  const stored = value as Record<string, unknown>;
  const activeProvider = migrateModelProvider(stored.activeProvider ?? stored.provider);
  if (stored.providers && typeof stored.providers === 'object') {
    const providers = stored.providers as Record<string, unknown>;
    return {
      activeProvider,
      providers: {
        openai: readProviderConfig(providers.openai),
        anthropic: readProviderConfig(providers.anthropic),
      },
    };
  }

  return {
    activeProvider,
    providers: {
      ...empty.providers,
      [activeProvider]: readProviderConfig(stored),
    },
  };
}

export function getActiveModelConfig(settings: ModelSettings): ModelConfig {
  return {
    provider: settings.activeProvider,
    ...settings.providers[settings.activeProvider],
  };
}
