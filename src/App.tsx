import {
  ArrowRight,
  BookOpen,
  Bot,
  Check,
  ChevronLeft,
  ChevronRight,
  CircleUserRound,
  FilePlus2,
  Activity,
  Dumbbell,
  FolderOpen,
  Globe2,
  History,
  House,
  Leaf,
  Library,
  LoaderCircle,
  MessageCircleMore,
  Minus,
  Monitor,
  Moon,
  NotebookPen,
  Pill,
  Download,
  Send,
  Settings,
  ShieldCheck,
  Square,
  Sparkles,
  Star,
  Sun,
  Trash2,
  Utensils,
  UserRound,
  UsersRound,
  Wrench,
  X,
} from 'lucide-react';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { FormEvent, ReactNode, useEffect, useMemo, useRef, useState } from 'react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import packageMetadata from '../package.json';
import {
  checkForUpdate,
  chooseKnowledgeFolder,
  downloadAndInstallUpdate,
  isTauri,
  loadLibrary,
  onSelfUpdateProgress,
  openExternalUrl,
  prepareCapture,
  readNote,
  saveCapture,
  sendAgentMessage,
  listenAgentEvents,
  resetAgent,
} from './api';
import { fallbackLibrary, fallbackMarkdown } from './data';
import { translate, type TranslationKey } from './i18n';
import type {
  AgentEvent,
  ChatMessage,
  CaptureDraft,
  LibrarySnapshot,
  Locale,
  ModelConfig,
  Person,
  Story,
  Supplement,
  View,
} from './types';

const APP_VERSION = packageMetadata.version;
const PRODUCT_WEBSITE = 'https://edison7009.github.io/OpenLongevity/';
const FEEDBACK_URL = 'https://github.com/edison7009/OpenLongevity/issues';

function createAmbientAssignments(count: number) {
  const assignments: Array<{
    delay: string;
    duration: string;
    direction: 'alternate' | 'alternate-reverse';
    secondaryDelay: string;
    secondaryDuration: string;
    secondaryDirection: 'alternate' | 'alternate-reverse';
  }> = [];
  const durationOffset = Math.random() * 11;

  for (let index = 0; index < count; index += 1) {
    assignments.push({
      delay: `${(-4 - Math.random() * 17).toFixed(2)}s`,
      duration: `${(10.6 + ((durationOffset + index * 2.71) % 12.8)).toFixed(2)}s`,
      direction: Math.random() > 0.5 ? 'alternate' : 'alternate-reverse',
      secondaryDelay: `${(-3 - Math.random() * 19).toFixed(2)}s`,
      secondaryDuration: `${(14.2 + ((durationOffset + index * 3.17) % 13.6)).toFixed(2)}s`,
      secondaryDirection: Math.random() > 0.5 ? 'alternate' : 'alternate-reverse',
    });
  }

  return assignments;
}

const providerPresets: Record<
  ModelConfig['provider'],
  { baseUrl: string; model: string; label: Record<Locale, string> }
> = {
  openai: {
    baseUrl: 'https://api.openai.com/v1',
    model: 'gpt-5.5',
    label: { zh: 'OpenAI', en: 'OpenAI' },
  },
  deepseek: {
    baseUrl: 'https://api.deepseek.com/v1',
    model: 'deepseek-v4-pro',
    label: { zh: 'DeepSeek', en: 'DeepSeek' },
  },
  openrouter: {
    baseUrl: 'https://openrouter.ai/api/v1',
    model: 'kimi-k3',
    label: { zh: 'OpenRouter', en: 'OpenRouter' },
  },
  custom: { baseUrl: '', model: '', label: { zh: '自定义', en: 'Custom' } },
};

const previousDefaultModels: Record<ModelConfig['provider'], string> = {
  openai: 'gpt-5-mini',
  deepseek: 'deepseek-chat',
  openrouter: 'openai/gpt-5-mini',
  custom: '',
};

const isMacOSPlatform =
  typeof navigator !== 'undefined' && /Macintosh|Mac OS X/.test(navigator.userAgent);

const tierMeta: Record<string, { label: Record<Locale, string>; color: string }> = {
  T1: { label: { zh: '基础支柱', en: 'Foundation' }, color: '#f27c78' },
  T2: { label: { zh: '高价值', en: 'High value' }, color: '#efb06e' },
  T3: { label: { zh: '条件明确', en: 'Targeted' }, color: '#efd269' },
  T4: { label: { zh: '特定情境', en: 'Contextual' }, color: '#9fcfc1' },
  T5: { label: { zh: '探索跟踪', en: 'Exploratory' }, color: '#b7dd91' },
  pending: { label: { zh: '待整理', en: 'Inbox' }, color: '#c8d3df' },
};

function useStoredState<T>(key: string, initial: T): [T, (value: T) => void] {
  const [state, setState] = useState<T>(() => {
    try {
      const stored = localStorage.getItem(key);
      return stored ? (JSON.parse(stored) as T) : initial;
    } catch {
      return initial;
    }
  });

  const update = (value: T) => {
    setState(value);
    try {
      localStorage.setItem(key, JSON.stringify(value));
    } catch {
      // Private browsing and full storage should not stop the app.
    }
  };
  return [state, update];
}

function normalizeMarkdown(markdown: string): string {
  const withoutFrontmatter = markdown.replace(
    /^\uFEFF?---[ \t]*\r?\n[\s\S]*?\r?\n---[ \t]*(?:\r?\n|$)/,
    '',
  );

  return withoutFrontmatter
    .replace(
    /::: tip\s*([^\n]*)\n([\s\S]*?)\n:::/g,
      (_match, title: string, body: string) => {
        const normalizedTitle = title.trim().toLowerCase();
        const visibleTitle =
          title.trim() === '30 秒结论' ||
          normalizedTitle === '30-second summary' ||
          normalizedTitle === '30-second conclusion'
            ? ''
            : title.trim();
        const heading = visibleTitle ? `> **${visibleTitle}**\n>\n` : '';
        return `${heading}${body
          .split('\n')
          .map((line) => `> ${line}`)
          .join('\n')}`;
      },
    )
    .replace(
      /^> \*\*(?:30 秒结论|30-second (?:summary|conclusion))\*\*\r?\n>\s*\r?\n/gim,
      '',
    );
}

function getLinks(markdown: string): Array<{ label: string; url: string }> {
  const links: Array<{ label: string; url: string }> = [];
  const pattern = /\[([^\]]+)\]\((https?:\/\/[^)]+)\)/g;
  for (const match of markdown.matchAll(pattern)) {
    if (!links.some((link) => link.url === match[2])) {
      links.push({ label: match[1], url: match[2] });
    }
  }
  return links.slice(0, 8);
}

type InternalNoteKind = 'supplement' | 'person' | 'story';
type PlanSection = 'supplements' | 'exercise' | 'diet' | 'sleep' | 'log';
type ThemeMode = 'system' | 'light' | 'dark';
type ResizeSide = 'left' | 'right';

interface PaneSizes {
  left: number;
  right: number;
}

const defaultPaneSizes: PaneSizes = { left: 248, right: 326 };

interface InternalNoteTarget {
  kind: InternalNoteKind;
  id: string;
  label: string;
}

interface FavoriteReference {
  kind: InternalNoteKind;
  id: string;
  addedAt: number;
}

interface FavoriteListItem {
  target: Omit<InternalNoteTarget, 'label'>;
  title: string;
  detail: string;
}

interface NavigationLocation {
  view: View;
  supplementId?: string;
  personId?: string;
  storyId?: string;
}

const FAVORITES_SEED_FLAG = 'openlongevity:favorites-seeded:v1';
const DEFAULT_FAVORITES: FavoriteReference[] = [
  { kind: 'person', id: 'bryan-johnson', addedAt: 0 },
];

type HealthLogField = 'exercise' | 'diet' | 'body';

interface HealthDayEntry {
  exercise?: string;
  diet?: string;
  body?: string;
}

type HealthLog = Record<string, HealthDayEntry>;

const HEALTH_LOG_KEY = 'openlongevity:health-log:v1';

function pad2(value: number): string {
  return String(value).padStart(2, '0');
}

function todayKey(): string {
  const d = new Date();
  return d.getFullYear() + '-' + pad2(d.getMonth() + 1) + '-' + pad2(d.getDate());
}

function shiftKey(key: string, delta: number): string {
  const parts = key.split('-').map(Number);
  const dt = new Date(parts[0], parts[1] - 1, parts[2] + delta);
  return dt.getFullYear() + '-' + pad2(dt.getMonth() + 1) + '-' + pad2(dt.getDate());
}

function entryHasContent(entry: HealthDayEntry | undefined): boolean {
  return Boolean(entry && (entry.exercise || entry.diet || entry.body));
}

function getPlanSections(locale: Locale): Array<{
  id: PlanSection;
  title: string;
  description: string;
  icon: ReactNode;
  accent: string;
}> {
  return [
    {
      id: 'supplements',
      title: locale === 'zh' ? '补剂计划' : 'Supplement plan',
      description:
        locale === 'zh'
          ? '成分、剂量、频率与安全检查'
          : 'Ingredients, dose, schedule, and safety',
      icon: <Pill size={17} />,
      accent: '#f1d9d5',
    },
    {
      id: 'exercise',
      title: locale === 'zh' ? '运动计划' : 'Exercise plan',
      description:
        locale === 'zh'
          ? '力量、有氧、活动量与恢复'
          : 'Strength, cardio, activity, and recovery',
      icon: <Dumbbell size={17} />,
      accent: '#d7e9e5',
    },
    {
      id: 'diet',
      title: locale === 'zh' ? '饮食计划' : 'Diet plan',
      description:
        locale === 'zh'
          ? '饮食结构、蛋白质与执行记录'
          : 'Eating pattern, protein, and adherence',
      icon: <Utensils size={17} />,
      accent: '#efe4c9',
    },
    {
      id: 'sleep',
      title: locale === 'zh' ? '睡眠计划' : 'Sleep plan',
      description:
        locale === 'zh'
          ? '作息、睡眠质量与恢复趋势'
          : 'Schedule, sleep quality, and recovery',
      icon: <Moon size={17} />,
      accent: '#dce3f2',
    },
    {
      id: 'log',
      title: locale === 'zh' ? '健康记录' : 'Health log',
      description:
        locale === 'zh'
          ? '按天记录运动、饮食与身体数据'
          : 'Daily notes for movement, food, and body',
      icon: <NotebookPen size={17} />,
      accent: '#cfeae3',
    },
  ];
}

function parseInternalNoteLink(href?: string): Omit<InternalNoteTarget, 'label'> | null {
  if (!href) return null;
  const match = href.match(/^#\/(supplement|person|story)\/([^/?#]+)$/);
  if (!match) return null;
  return {
    kind: match[1] as InternalNoteKind,
    id: decodeURIComponent(match[2]),
  };
}

function AppLink({
  href,
  children,
  onClick,
  onInternalNavigate,
  node: _node,
  ...props
}: React.AnchorHTMLAttributes<HTMLAnchorElement> & {
  node?: unknown;
  onInternalNavigate?: (target: Omit<InternalNoteTarget, 'label'>) => void;
}) {
  const internalTarget = parseInternalNoteLink(href);
  const external = Boolean(href && /^https?:\/\//i.test(href));
  const className = [props.className, internalTarget ? 'internal-note-link' : '']
    .filter(Boolean)
    .join(' ');

  return (
    <a
      {...props}
      href={href}
      className={className || undefined}
      rel={external ? 'noreferrer' : props.rel}
      onClick={(event) => {
        onClick?.(event);
        if (event.defaultPrevented) return;
        if (internalTarget && onInternalNavigate) {
          event.preventDefault();
          onInternalNavigate(internalTarget);
          return;
        }
        if (!external || !href) return;
        event.preventDefault();
        void openExternalUrl(href);
      }}
    >
      {children}
    </a>
  );
}

function isAsciiWordCharacter(value?: string): boolean {
  return Boolean(value && /[A-Za-z0-9_]/.test(value));
}

function findTermIndex(text: string, label: string): number {
  const haystack = text.toLocaleLowerCase();
  const needle = label.toLocaleLowerCase();
  let from = 0;

  while (from <= haystack.length - needle.length) {
    const index = haystack.indexOf(needle, from);
    if (index < 0) return -1;
    const before = text[index - 1];
    const after = text[index + label.length];
    const startsWithWord = isAsciiWordCharacter(label[0]);
    const endsWithWord = isAsciiWordCharacter(label[label.length - 1]);
    if (
      (!startsWithWord || !isAsciiWordCharacter(before)) &&
      (!endsWithWord || !isAsciiWordCharacter(after))
    ) {
      return index;
    }
    from = index + 1;
  }

  return -1;
}

function linkInternalKeywords(
  markdown: string,
  targets: InternalNoteTarget[],
  currentTarget: Omit<InternalNoteTarget, 'label'>,
): string {
  const currentKey = `${currentTarget.kind}:${currentTarget.id}`;
  const seenLabels = new Set<string>();
  const candidates = targets
    .filter((target) => `${target.kind}:${target.id}` !== currentKey)
    .filter((target) => {
      const label = target.label.trim().toLocaleLowerCase();
      if (!label || seenLabels.has(label)) return false;
      seenLabels.add(label);
      return true;
    })
    .sort((left, right) => right.label.length - left.label.length);
  const linkedTargets = new Set<string>();
  let inFence = false;

  const linkText = (text: string) => {
    let remaining = text;
    let output = '';

    while (remaining) {
      let next:
        | {
            target: InternalNoteTarget;
            index: number;
          }
        | undefined;

      for (const target of candidates) {
        const key = `${target.kind}:${target.id}`;
        if (linkedTargets.has(key)) continue;
        const index = findTermIndex(remaining, target.label);
        if (
          index >= 0 &&
          (!next ||
            index < next.index ||
            (index === next.index && target.label.length > next.target.label.length))
        ) {
          next = { target, index };
        }
      }

      if (!next) {
        output += remaining;
        break;
      }

      const matchedText = remaining.slice(next.index, next.index + next.target.label.length);
      output += remaining.slice(0, next.index);
      output += `[${matchedText}](#/${next.target.kind}/${encodeURIComponent(next.target.id)})`;
      linkedTargets.add(`${next.target.kind}:${next.target.id}`);
      remaining = remaining.slice(next.index + next.target.label.length);
    }

    return output;
  };

  return markdown
    .split('\n')
    .map((line) => {
      if (/^\s*(```|~~~)/.test(line)) {
        inFence = !inFence;
        return line;
      }
      if (inFence || /^\s{0,3}#{1,6}\s/.test(line)) return line;

      const protectedMarkdown =
        /(`[^`]*`|!?\[[^\]]*\]\([^)]*\)|https?:\/\/[^\s<]+|<[^>]+>)/g;
      let output = '';
      let cursor = 0;
      for (const match of line.matchAll(protectedMarkdown)) {
        const index = match.index ?? 0;
        output += linkText(line.slice(cursor, index));
        output += match[0];
        cursor = index + match[0].length;
      }
      output += linkText(line.slice(cursor));
      return output;
    })
    .join('\n');
}

function locationsMatch(left: NavigationLocation, right: NavigationLocation): boolean {
  return (
    left.view === right.view &&
    left.supplementId === right.supplementId &&
    left.personId === right.personId &&
    left.storyId === right.storyId
  );
}

type ToastState = {
  message: string;
  kind: 'status' | 'favorite-added' | 'favorite-removed';
};

function App() {
  const [locale, setLocale] = useStoredState<Locale>('openlongevity:locale', 'zh');
  const [themeMode, setThemeMode] = useStoredState<ThemeMode>(
    'openlongevity:theme',
    'system',
  );
  const [paneSizes, setPaneSizes] = useStoredState<PaneSizes>(
    'openlongevity:pane-sizes',
    defaultPaneSizes,
  );
  const [favorites, setFavorites] = useStoredState<FavoriteReference[]>(
    'openlongevity:favorites',
    [],
  );
  const [knowledgeRoot, setKnowledgeRoot] = useStoredState(
    'openlongevity:knowledge-root:v2',
    '',
  );
  const [modelDiskConfig, setModelDiskConfig] = useStoredState<Omit<ModelConfig, 'apiKey'>>(
    'openlongevity:model',
    {
      provider: 'openai',
      baseUrl: providerPresets.openai.baseUrl,
      model: providerPresets.openai.model,
    },
  );
  const [apiKey, setApiKey] = useState('');
  const [library, setLibrary] = useState<LibrarySnapshot>(fallbackLibrary);
  const [loadingLibrary, setLoadingLibrary] = useState(true);
  const [view, setView] = useState<View>('home');
  const [selectedSupplement, setSelectedSupplement] = useState<Supplement | null>(null);
  const [selectedPerson, setSelectedPerson] = useState<Person | null>(null);
  const [selectedStory, setSelectedStory] = useState<Story | null>(null);
  const [activePlanSection, setActivePlanSection] = useState<PlanSection>('supplements');
  const [noteMarkdown, setNoteMarkdown] = useState('');
  const [noteLoading, setNoteLoading] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [captureGuideOpen, setCaptureGuideOpen] = useState(false);
  const [chatMessages, setChatMessages] = useState<ChatMessage[]>([]);
  const [chatActive, setChatActive] = useState(false);
  const [chatBusy, setChatBusy] = useState(false);
  const [toast, setToast] = useState<ToastState | null>(null);
  const [resizingPane, setResizingPane] = useState<ResizeSide | null>(null);
  const chatComposerRef = useRef<HTMLTextAreaElement>(null);
  const navigationHistoryRef = useRef<NavigationLocation[]>([]);

  useEffect(() => {
    let seeded = false;
    try {
      seeded = window.localStorage.getItem(FAVORITES_SEED_FLAG) === '1';
    } catch {
      seeded = false;
    }
    if (!seeded) {
      if (favorites.length === 0) {
        setFavorites(DEFAULT_FAVORITES);
      }
      try {
        window.localStorage.setItem(FAVORITES_SEED_FLAG, '1');
      } catch {
        // ignore unavailable storage
      }
    }
    // Seed the default favorite once on first launch; never overwrite later edits.
  }, []);

  const t = (key: TranslationKey) => translate(locale, key);
  const modelConfig: ModelConfig = { ...modelDiskConfig, apiKey };

  useEffect(() => {
    const systemTheme = window.matchMedia('(prefers-color-scheme: dark)');
    const applyTheme = () => {
      const resolvedTheme =
        themeMode === 'system' ? (systemTheme.matches ? 'dark' : 'light') : themeMode;
      document.documentElement.dataset.theme = resolvedTheme;
      document.documentElement.style.colorScheme = resolvedTheme;
    };

    applyTheme();
    systemTheme.addEventListener('change', applyTheme);
    return () => systemTheme.removeEventListener('change', applyTheme);
  }, [themeMode]);

  useEffect(() => {
    document.documentElement.lang = locale === 'zh' ? 'zh-CN' : 'en';
  }, [locale]);

  const resizePane = (side: ResizeSide, requestedSize: number) => {
    const viewportWidth = window.innerWidth;
    const visibleRightWidth = viewportWidth <= 1120 ? 0 : paneSizes.right;
    const maximum =
      side === 'left'
        ? Math.max(210, Math.min(380, viewportWidth - visibleRightWidth - 560))
        : Math.max(270, Math.min(460, viewportWidth - paneSizes.left - 560));
    const minimum = side === 'left' ? 210 : 270;
    const nextSize = Math.round(Math.min(maximum, Math.max(minimum, requestedSize)));
    setPaneSizes({ ...paneSizes, [side]: nextSize });
  };

  const getCurrentLocation = (): NavigationLocation => ({
    view,
    supplementId: selectedSupplement?.id,
    personId: selectedPerson?.id,
    storyId: selectedStory?.id,
  });

  const rememberCurrentLocation = (nextLocation: NavigationLocation) => {
    const currentLocation = getCurrentLocation();
    if (!locationsMatch(currentLocation, nextLocation)) {
      navigationHistoryRef.current.push(currentLocation);
    }
  };

  useEffect(() => {
    const previousDefault = previousDefaultModels[modelDiskConfig.provider];
    const nextDefault = providerPresets[modelDiskConfig.provider].model;
    if (modelDiskConfig.model === previousDefault && nextDefault !== previousDefault) {
      setModelDiskConfig({ ...modelDiskConfig, model: nextDefault });
    }
  }, []);

  useEffect(() => {
    let alive = true;
    setLoadingLibrary(true);
    loadLibrary(knowledgeRoot || undefined, locale)
      .then((snapshot) => {
        if (!alive) return;
        setLibrary(snapshot);
        if (!knowledgeRoot && snapshot.root) setKnowledgeRoot(snapshot.root);
      })
      .catch(() => {
        if (alive) setLibrary(fallbackLibrary);
      })
      .finally(() => {
        if (alive) setLoadingLibrary(false);
      });
    return () => {
      alive = false;
    };
  }, [knowledgeRoot, locale]);

  useEffect(() => {
    if (!toast) return;
    const timer = window.setTimeout(() => setToast(null), 2400);
    return () => window.clearTimeout(timer);
  }, [toast]);

  const openSupplement = async (supplement: Supplement, remember = true) => {
    if (remember) {
      rememberCurrentLocation({ view: 'supplement', supplementId: supplement.id });
    }
    setSelectedSupplement(supplement);
    setSelectedPerson(null);
    setSelectedStory(null);
    setView('supplement');
    setChatActive(false);
    setNoteLoading(true);
    try {
      const markdown = supplement.filePath
        ? await readNote(library.root, supplement.filePath)
        : `# ${supplement.nameZh}\n\n${supplement.summary}`;
      setNoteMarkdown(markdown);
    } catch {
      setNoteMarkdown(
        fallbackMarkdown[supplement.filePath || ''] ||
          `# ${supplement.nameZh}\n\n${supplement.summary}`,
      );
    } finally {
      setNoteLoading(false);
    }
  };

  const openPerson = async (person: Person, remember = true) => {
    if (remember) {
      rememberCurrentLocation({ view: 'person', personId: person.id });
    }
    setSelectedPerson(person);
    setSelectedSupplement(null);
    setSelectedStory(null);
    setView('person');
    setChatActive(false);
    setNoteLoading(true);
    try {
      const markdown = person.filePath
        ? await readNote(library.root, person.filePath)
        : `# ${person.name}\n\n${person.summary}`;
      setNoteMarkdown(markdown);
    } catch {
      setNoteMarkdown(
        fallbackMarkdown[person.filePath || ''] || `# ${person.name}\n\n${person.summary}`,
      );
    } finally {
      setNoteLoading(false);
    }
  };

  const openStory = async (story: Story, remember = true) => {
    if (remember) {
      rememberCurrentLocation({ view: 'story', storyId: story.id });
    }
    setSelectedStory(story);
    setSelectedSupplement(null);
    setSelectedPerson(null);
    setView('story');
    setChatActive(false);
    setNoteLoading(true);
    try {
      const markdown = story.filePath
        ? await readNote(library.root, story.filePath)
        : `# ${story.title}\n\n${story.summary}`;
      setNoteMarkdown(markdown);
    } catch {
      setNoteMarkdown(
        fallbackMarkdown[story.filePath || ''] || `# ${story.title}\n\n${story.summary}`,
      );
    } finally {
      setNoteLoading(false);
    }
  };

  useEffect(() => {
    if (view === 'supplement' && selectedSupplement) {
      const localized = library.supplements.find((item) => item.id === selectedSupplement.id);
      if (localized && localized.filePath !== selectedSupplement.filePath) {
        void openSupplement(localized, false);
      }
    } else if (view === 'person' && selectedPerson) {
      const localized = library.people.find((item) => item.id === selectedPerson.id);
      if (localized && localized.filePath !== selectedPerson.filePath) {
        void openPerson(localized, false);
      }
    } else if (view === 'story' && selectedStory) {
      const localized = library.stories.find((item) => item.id === selectedStory.id);
      if (localized && localized.filePath !== selectedStory.filePath) {
        void openStory(localized, false);
      }
    }
  }, [library, locale]);

  const navigate = (nextView: View, remember = true) => {
    if (remember) rememberCurrentLocation({ view: nextView });
    setView(nextView);
    setChatActive(false);
    if (nextView !== 'supplement') setSelectedSupplement(null);
    if (nextView !== 'person') setSelectedPerson(null);
    if (nextView !== 'story') setSelectedStory(null);
    if (!['supplement', 'person', 'story'].includes(nextView)) setNoteMarkdown('');
  };

  const restoreLocation = (location: NavigationLocation) => {
    if (location.view === 'supplement' && location.supplementId) {
      const supplement = library.supplements.find((item) => item.id === location.supplementId);
      if (supplement) {
        void openSupplement(supplement, false);
        return;
      }
    }
    if (location.view === 'person' && location.personId) {
      const person = library.people.find((item) => item.id === location.personId);
      if (person) {
        void openPerson(person, false);
        return;
      }
    }
    if (location.view === 'story' && location.storyId) {
      const story = library.stories.find((item) => item.id === location.storyId);
      if (story) {
        void openStory(story, false);
        return;
      }
    }
    navigate(location.view, false);
  };

  const goBack = () => {
    const previous = navigationHistoryRef.current.pop();
    restoreLocation(previous || { view: 'home' });
  };

  const openPlanSection = (section: PlanSection) => {
    setActivePlanSection(section);
    navigate('plan');
  };

  const openInternalNote = (target: Omit<InternalNoteTarget, 'label'>) => {
    if (target.kind === 'supplement') {
      const supplement = library.supplements.find((item) => item.id === target.id);
      if (supplement) {
        void openSupplement(supplement);
        return;
      }
    }
    if (target.kind === 'person') {
      const person = library.people.find((item) => item.id === target.id);
      if (person) {
        void openPerson(person);
        return;
      }
    }
    if (target.kind === 'story') {
      const story = library.stories.find((item) => item.id === target.id);
      if (story) {
        void openStory(story);
        return;
      }
    }
    setToast({
      message: locale === 'zh' ? '没有找到对应的本地文章' : 'The linked local note was not found',
      kind: 'status',
    });
  };

  const isFavorite = (target: Omit<InternalNoteTarget, 'label'>) =>
    favorites.some((favorite) => favorite.kind === target.kind && favorite.id === target.id);

  const toggleFavorite = (target: Omit<InternalNoteTarget, 'label'>) => {
    if (isFavorite(target)) {
      setFavorites(
        favorites.filter(
          (favorite) => favorite.kind !== target.kind || favorite.id !== target.id,
        ),
      );
      setToast({ message: t('favoriteRemoved'), kind: 'favorite-removed' });
      return;
    }

    setFavorites([{ ...target, addedAt: Date.now() }, ...favorites]);
    setToast({ message: t('favoriteAdded'), kind: 'favorite-added' });
  };

  const finishCapture = async (path: string) => {
    setCaptureGuideOpen(false);
    try {
      const snapshot = await loadLibrary(library.root || knowledgeRoot || undefined, locale);
      setLibrary(snapshot);
    } catch {
      // The note is already saved; a later library refresh can recover the updated count.
    }
    setToast({
      message:
        locale === 'zh'
          ? `已保存到本地收件箱：${path}`
          : `Saved to the local inbox: ${path}`,
      kind: 'status',
    });
  };

  // ── Agent event listener ──
  useEffect(() => {
    let unlisten: (() => void) | null = null;
    let cancelled = false;
    listenAgentEvents((event: AgentEvent) => {
      if (cancelled) return;
      switch (event.type) {
        case 'text_delta':
          setChatMessages((prev) => {
            const last = prev[prev.length - 1];
            if (last && last.role === 'assistant') {
              return [...prev.slice(0, -1), { ...last, content: last.content + event.text }];
            }
            return [...prev, { id: crypto.randomUUID(), role: 'assistant' as const, content: event.text, createdAt: Date.now() }];
          });
          break;
        case 'tool_call_start':
          setChatMessages((prev) => [
            ...prev,
            {
              id: event.id,
              role: 'tool_call' as const,
              content: '',
              createdAt: Date.now(),
              toolName: event.name,
              toolArgs: '',
              toolStatus: 'running' as const,
            },
          ]);
          break;
        case 'tool_call_args':
          setChatMessages((prev) =>
            prev.map((m) =>
              m.role === 'tool_call' && m.id === event.id
                ? { ...m, toolArgs: (m.toolArgs || '') + event.args }
                : m,
            ),
          );
          break;
        case 'tool_result':
          setChatMessages((prev) =>
            prev.map((m) =>
              m.role === 'tool_call' && m.id === event.id
                ? { ...m, toolStatus: event.success ? 'done' as const : 'failed' as const, toolOutput: event.output }
                : m,
            ),
          );
          break;
        case 'done':
          setChatBusy(false);
          break;
        case 'error':
          setChatMessages((prev) => [
            ...prev,
            {
              id: crypto.randomUUID(),
              role: 'assistant' as const,
              content: locale === 'zh' ? `错误：${event.message}` : `Error: ${event.message}`,
              createdAt: Date.now(),
            },
          ]);
          setChatBusy(false);
          break;
      }
    }).then((fn) => {
      if (cancelled) fn();
      else unlisten = fn;
    });
    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, [locale]);

  const handleSend = async (question: string) => {
    const clean = question.trim();
    if (!clean || chatBusy) return;

    const userMessage: ChatMessage = {
      id: crypto.randomUUID(),
      role: 'user',
      content: clean,
      createdAt: Date.now(),
    };
    const priorMessages = chatMessages;
    setChatMessages((current) => [...current, userMessage]);
    setChatActive(true);
    setChatBusy(true);

    if (isTauri && !modelConfig.apiKey) {
      const assistantMessage: ChatMessage = {
        id: crypto.randomUUID(),
        role: 'assistant',
        content: t('notConfigured'),
        createdAt: Date.now(),
      };
      setChatMessages((current) => [...current, assistantMessage]);
      setChatBusy(false);
      return;
    }

    try {
      await sendAgentMessage({
        apiKey: modelConfig.apiKey,
        baseUrl: modelConfig.baseUrl,
        model: modelConfig.model,
        message: clean,
        locale,
        knowledgeRoot: library.root,
        contextPaths: [
          selectedSupplement?.filePath,
          selectedPerson?.filePath,
          selectedStory?.filePath,
        ].filter(Boolean) as string[],
        history: priorMessages
          .filter((m) => m.role === 'user' || m.role === 'assistant')
          .slice(-8)
          .map(({ role, content: messageContent }) => ({
            role: role as 'user' | 'assistant',
            content: messageContent,
          })),
      });
    } catch (error) {
      setChatMessages((current) => [
        ...current,
        {
          id: crypto.randomUUID(),
          role: 'assistant',
          content:
            locale === 'zh'
              ? `模型请求失败：${String(error)}`
              : `Model request failed: ${String(error)}`,
          createdAt: Date.now(),
        },
      ]);
      setChatBusy(false);
    }
  };

  const handleNewChat = async () => {
    setChatMessages([]);
    setChatActive(false);
    try {
      await resetAgent();
    } catch {
      // ignore
    }
  };
  const changeKnowledgeRoot = async () => {
    const selected = await chooseKnowledgeFolder();
    if (selected) {
      setKnowledgeRoot(selected);
      setSettingsOpen(false);
      setToast({
        message: locale === 'zh' ? '正在切换本地知识库…' : 'Switching local library…',
        kind: 'status',
      });
    }
  };

  const saveModelConfig = (config: ModelConfig) => {
    setModelDiskConfig({
      provider: config.provider,
      baseUrl: config.baseUrl,
      model: config.model,
    });
    setApiKey(config.apiKey);
  };

  const references = useMemo(() => getLinks(noteMarkdown), [noteMarkdown]);
  const internalNoteTargets = useMemo<InternalNoteTarget[]>(() => {
    const targets: InternalNoteTarget[] = [];
    const add = (kind: InternalNoteKind, id: string, labels: Array<string | undefined>) => {
      for (const label of labels) {
        const clean = label?.trim();
        if (clean) targets.push({ kind, id, label: clean });
      }
    };

    for (const supplement of library.supplements) {
      add('supplement', supplement.id, [supplement.nameZh, supplement.nameEn]);
    }
    for (const person of library.people) {
      add('person', person.id, [person.name, person.nameZh]);
    }
    for (const story of library.stories) {
      add('story', story.id, [story.title, story.titleEn]);
    }
    return targets;
  }, [library]);

  return (
    <div
      className={`app-shell ${resizingPane ? 'panel-resizing' : ''}`}
      style={
        {
          '--sidebar-width': `${paneSizes.left}px`,
          '--right-rail-width': `${paneSizes.right}px`,
        } as React.CSSProperties
      }
    >
      <AppTitlebar locale={locale} onSettings={() => setSettingsOpen(true)} />
      <Sidebar
        locale={locale}
        library={library}
        view={view}
        selectedSupplement={selectedSupplement}
        selectedPerson={selectedPerson}
        selectedStory={selectedStory}
        onNavigate={navigate}
        onSupplement={openSupplement}
        onPerson={openPerson}
        onStory={openStory}
        t={t}
      />
      <PaneResizer
        side="left"
        size={paneSizes.left}
        locale={locale}
        onResize={(size) => resizePane('left', size)}
        onReset={() => setPaneSizes({ ...paneSizes, left: defaultPaneSizes.left })}
        onResizing={setResizingPane}
      />

      <main className="main-pane">
        <div className="content-scroll">
          {loadingLibrary ? (
            <div className="loading-state">
              <LoaderCircle className="spin" size={24} />
              <span>{t('loading')}</span>
            </div>
          ) : chatActive ? (
            <ConversationView
              locale={locale}
              messages={chatMessages}
              busy={chatBusy}
              onBack={() => setChatActive(false)}
              onNewChat={handleNewChat}
              onInternalNavigate={openInternalNote}
            />
          ) : (
            <>
              {view === 'home' && (
                <HomeView
                  locale={locale}
                  library={library}
                  onCapture={() => setCaptureGuideOpen(true)}
                  onPeople={() => navigate('people')}
                  onPlan={() => navigate('plan')}
                  onSupplement={openSupplement}
                  t={t}
                />
              )}
              {view === 'people' && (
                <PeopleView
                  locale={locale}
                  people={library.people}
                  onPerson={openPerson}
                  onBack={goBack}
                  t={t}
                />
              )}
              {view === 'stories' && (
                <StoriesView
                  locale={locale}
                  stories={library.stories}
                  onStory={openStory}
                  onAdd={() => setCaptureGuideOpen(true)}
                  onBack={goBack}
                  t={t}
                />
              )}
              {view === 'supplement' && selectedSupplement && (
                <NoteView
                  eyebrow={locale === 'zh' ? selectedSupplement.category : selectedSupplement.tier}
                  title={locale === 'zh' ? selectedSupplement.nameZh : selectedSupplement.nameEn}
                  tier={selectedSupplement.tier}
                  markdown={noteMarkdown}
                  loading={noteLoading}
                  locale={locale}
                  currentTarget={{ kind: 'supplement', id: selectedSupplement.id }}
                  internalTargets={internalNoteTargets}
                  onInternalNavigate={openInternalNote}
                  favorite={isFavorite({ kind: 'supplement', id: selectedSupplement.id })}
                  onToggleFavorite={() =>
                    toggleFavorite({ kind: 'supplement', id: selectedSupplement.id })
                  }
                  onBack={goBack}
                />
              )}
              {view === 'person' && selectedPerson && (
                <NoteView
                  eyebrow={locale === 'zh' ? '人物案例' : 'Public protocol'}
                  title={selectedPerson.name}
                  markdown={noteMarkdown}
                  loading={noteLoading}
                  locale={locale}
                  currentTarget={{ kind: 'person', id: selectedPerson.id }}
                  internalTargets={internalNoteTargets}
                  onInternalNavigate={openInternalNote}
                  favorite={isFavorite({ kind: 'person', id: selectedPerson.id })}
                  onToggleFavorite={() =>
                    toggleFavorite({ kind: 'person', id: selectedPerson.id })
                  }
                  onBack={goBack}
                />
              )}
              {view === 'story' && selectedStory && (
                <NoteView
                  eyebrow={t('stories')}
                  title={
                    locale === 'zh' ? selectedStory.title : selectedStory.titleEn || selectedStory.title
                  }
                  markdown={noteMarkdown}
                  loading={noteLoading}
                  locale={locale}
                  currentTarget={{ kind: 'story', id: selectedStory.id }}
                  internalTargets={internalNoteTargets}
                  onInternalNavigate={openInternalNote}
                  favorite={isFavorite({ kind: 'story', id: selectedStory.id })}
                  onToggleFavorite={() =>
                    toggleFavorite({ kind: 'story', id: selectedStory.id })
                  }
                  onBack={goBack}
                />
              )}
              {view === 'plan' && (
                <PlanView
                  locale={locale}
                  activeSection={activePlanSection}
                  onSection={setActivePlanSection}
                  onBack={goBack}
                  t={t}
                />
              )}
            </>
          )}
        </div>

        <ChatComposer
          busy={chatBusy}
          onSend={handleSend}
          placeholder={t('askPlaceholder')}
          inputRef={chatComposerRef}
        />
      </main>

      <PaneResizer
        side="right"
        size={paneSizes.right}
        locale={locale}
        onResize={(size) => resizePane('right', size)}
        onReset={() => setPaneSizes({ ...paneSizes, right: defaultPaneSizes.right })}
        onResizing={setResizingPane}
      />

      <RightRail
        locale={locale}
        view={view}
        chatActive={chatActive}
        messages={chatMessages}
        supplement={selectedSupplement}
        person={selectedPerson}
        story={selectedStory}
        references={references}
        library={library}
        favorites={favorites}
        activePlanSection={activePlanSection}
        onFavoriteNavigate={openInternalNote}
        onPlanSection={openPlanSection}
        onResumeChat={() => setChatActive(true)}
        t={t}
      />

      {settingsOpen && (
        <SettingsDialog
          locale={locale}
          config={modelConfig}
          knowledgeRoot={library.root || knowledgeRoot}
          onChange={saveModelConfig}
          onLocale={setLocale}
          themeMode={themeMode}
          onThemeMode={setThemeMode}
          onChooseFolder={changeKnowledgeRoot}
          onClose={() => setSettingsOpen(false)}
          t={t}
        />
      )}

      {captureGuideOpen && (
        <CaptureGuideDialog
          locale={locale}
          config={modelConfig}
          knowledgeRoot={library.root || knowledgeRoot}
          onClose={() => setCaptureGuideOpen(false)}
          onSaved={finishCapture}
          t={t}
        />
      )}

      {toast && (
        <div className={`toast ${toast.kind}`} role="status" aria-live="polite">
          {toast.kind === 'favorite-added' ? (
            <Star size={17} fill="currentColor" />
          ) : toast.kind === 'favorite-removed' ? (
            <Star size={17} />
          ) : (
            <Check size={17} />
          )}
          <span>{toast.message}</span>
        </div>
      )}
    </div>
  );
}

function PaneResizer({
  side,
  size,
  locale,
  onResize,
  onReset,
  onResizing,
}: {
  side: ResizeSide;
  size: number;
  locale: Locale;
  onResize: (size: number) => void;
  onReset: () => void;
  onResizing: (side: ResizeSide | null) => void;
}) {
  const startResize = (event: React.PointerEvent<HTMLDivElement>) => {
    if (event.button !== 0) return;
    event.preventDefault();

    const startX = event.clientX;
    const startSize = size;
    onResizing(side);
    document.body.classList.add('resizing-panels');

    const handleMove = (moveEvent: PointerEvent) => {
      const delta = moveEvent.clientX - startX;
      onResize(side === 'left' ? startSize + delta : startSize - delta);
    };
    const finish = () => {
      window.removeEventListener('pointermove', handleMove);
      window.removeEventListener('pointerup', finish);
      window.removeEventListener('pointercancel', finish);
      document.body.classList.remove('resizing-panels');
      onResizing(null);
    };

    window.addEventListener('pointermove', handleMove);
    window.addEventListener('pointerup', finish);
    window.addEventListener('pointercancel', finish);
  };

  return (
    <div
      className={`pane-resizer pane-resizer-${side}`}
      role="separator"
      aria-orientation="vertical"
      aria-label={
        locale === 'zh'
          ? side === 'left'
            ? '调整左侧栏宽度'
            : '调整右侧栏宽度'
          : side === 'left'
            ? 'Resize left panel'
            : 'Resize right panel'
      }
      onPointerDown={startResize}
      onDoubleClick={onReset}
    >
      <span />
    </div>
  );
}

function AppTitlebar({ locale, onSettings }: { locale: Locale; onSettings: () => void }) {
  const runWindowCommand = async (command: 'minimize' | 'maximize' | 'close') => {
    if (!isTauri) return;

    const appWindow = getCurrentWindow();
    if (command === 'minimize') {
      await appWindow.minimize();
    } else if (command === 'maximize') {
      await appWindow.toggleMaximize();
    } else {
      await appWindow.close();
    }
  };

  return (
    <header
      className={`app-titlebar ${isMacOSPlatform ? 'platform-macos' : 'platform-custom-controls'}`}
      data-tauri-drag-region
    >
      <div className="titlebar-drag-area" data-tauri-drag-region />

      <div className="window-controls">
        <button
          type="button"
          className="titlebar-settings"
          aria-label={locale === 'zh' ? '打开设置' : 'Open settings'}
          onClick={onSettings}
        >
          <Settings size={14} strokeWidth={1.8} />
        </button>
        {!isMacOSPlatform && (
          <>
            <button
              type="button"
              aria-label={locale === 'zh' ? '最小化窗口' : 'Minimize window'}
              onClick={() => void runWindowCommand('minimize')}
            >
              <Minus size={15} strokeWidth={1.8} />
            </button>
            <button
              type="button"
              aria-label={locale === 'zh' ? '最大化或还原窗口' : 'Maximize or restore window'}
              onClick={() => void runWindowCommand('maximize')}
            >
              <Square size={11} strokeWidth={1.8} />
            </button>
            <button
              type="button"
              className="window-close"
              aria-label={locale === 'zh' ? '关闭窗口' : 'Close window'}
              onClick={() => void runWindowCommand('close')}
            >
              <X size={15} strokeWidth={1.8} />
            </button>
          </>
        )}
      </div>
    </header>
  );
}

interface SidebarProps {
  locale: Locale;
  library: LibrarySnapshot;
  view: View;
  selectedSupplement: Supplement | null;
  selectedPerson: Person | null;
  selectedStory: Story | null;
  onNavigate: (view: View) => void;
  onSupplement: (supplement: Supplement) => void;
  onPerson: (person: Person) => void;
  onStory: (story: Story) => void;
  t: (key: TranslationKey) => string;
}

function Sidebar({
  locale,
  library,
  view,
  selectedSupplement,
  selectedPerson,
  selectedStory,
  onNavigate,
  onSupplement,
  onPerson,
  onStory,
  t,
}: SidebarProps) {
  const mainSupplements = library.supplements;
  const [strategiesExpanded, setStrategiesExpanded] = useState(true);
  const [peopleExpanded, setPeopleExpanded] = useState(true);
  const [storiesExpanded, setStoriesExpanded] = useState(true);
  const [availableVersion, setAvailableVersion] = useState<string | null>(null);
  const [installingUpdate, setInstallingUpdate] = useState(false);
  const [updateProgress, setUpdateProgress] = useState(0);
  const [updatePhase, setUpdatePhase] = useState<
    'checking' | 'downloading' | 'launching' | 'error' | null
  >(null);

  useEffect(() => {
    let alive = true;
    if (isTauri) {
      checkForUpdate()
        .then((version) => {
          if (alive) setAvailableVersion(version);
        })
        .catch(() => {
          // Update checks stay silent so an offline launch is never interrupted.
        });
    }
    return () => {
      alive = false;
    };
  }, []);

  const handleUpdate = async () => {
    if (installingUpdate) return;
    const isWindows = navigator.userAgent.toLowerCase().includes('windows');
    if (!isTauri || !isWindows) {
      await openExternalUrl(PRODUCT_WEBSITE);
      return;
    }

    setInstallingUpdate(true);
    setUpdateProgress(0);
    setUpdatePhase('checking');
    let stopListening: (() => void) | undefined;
    try {
      stopListening = await onSelfUpdateProgress((progress) => {
        setUpdatePhase(progress.status);
        setUpdateProgress(progress.percent);
      });
      await downloadAndInstallUpdate();
    } catch {
      setUpdatePhase('error');
      try {
        await openExternalUrl(PRODUCT_WEBSITE);
      } finally {
        setInstallingUpdate(false);
        setUpdateProgress(0);
      }
    } finally {
      stopListening?.();
    }
  };

  useEffect(() => {
    if (selectedSupplement) setStrategiesExpanded(true);
  }, [selectedSupplement]);

  useEffect(() => {
    if (selectedPerson) setPeopleExpanded(true);
  }, [selectedPerson]);

  useEffect(() => {
    if (selectedStory) setStoriesExpanded(true);
  }, [selectedStory]);

  return (
    <aside className="sidebar">
      <div className="sidebar-scroll">
        <div className="brand">
          <button className="brand-main" onClick={() => onNavigate('home')}>
            <img src="/brand/logo.png" alt="" />
            <span>
              <strong>{t('appName')}</strong>
              <small>{t('appTagline')}</small>
            </span>
          </button>
          {availableVersion && (
            <button
              className={`brand-update ${installingUpdate ? 'installing' : ''}`}
              onClick={() => void handleUpdate()}
              aria-label={
                locale === 'zh'
                  ? `更新至 Open Longevity ${availableVersion}`
                  : `Update Open Longevity to ${availableVersion}`
              }
              disabled={updatePhase === 'launching'}
            >
              {installingUpdate ? (
                <svg className="update-ring" viewBox="0 0 24 24" aria-hidden="true">
                  <circle className="update-ring-track" cx="12" cy="12" r="9" />
                  <circle
                    className={`update-ring-progress ${
                      updatePhase === 'checking' ? 'indeterminate' : ''
                    }`}
                    cx="12"
                    cy="12"
                    r="9"
                    pathLength="100"
                    style={{ strokeDashoffset: 100 - updateProgress }}
                  />
                </svg>
              ) : (
                <Download size={15} strokeWidth={2} />
              )}
            </button>
          )}
        </div>

        <nav className="primary-nav" aria-label="Primary">
          <SidebarButton
            icon={<House size={17} />}
            label={t('home')}
            active={view === 'home'}
            onClick={() => onNavigate('home')}
          />
          <SidebarButton
            icon={<Sparkles size={17} />}
            label={t('myPlan')}
            active={view === 'plan'}
            onClick={() => onNavigate('plan')}
          />

          <div className="nav-divider" />

          <div className="nav-tree-group">
            <button
              className={`nav-button nav-tree-toggle ${view === 'supplement' ? 'active' : ''}`}
              onClick={() => setStrategiesExpanded((expanded) => !expanded)}
              aria-expanded={strategiesExpanded}
            >
              <Library size={17} />
              <span>{t('supplementLibrary')}</span>
              <small className="nav-count">{library.supplements.length}</small>
              <ChevronRight
                size={14}
                className={`tree-chevron ${strategiesExpanded ? 'expanded' : ''}`}
              />
            </button>

            {strategiesExpanded && (
              <div className="tree-children">
                {mainSupplements.map((supplement) => (
                  <button
                    key={supplement.id}
                    className={`tree-child ${
                      selectedSupplement?.id === supplement.id ? 'active' : ''
                    }`}
                    onClick={() => onSupplement(supplement)}
                  >
                    <span>{locale === 'zh' ? supplement.nameZh : supplement.nameEn}</span>
                    <small>{supplement.tier}</small>
                  </button>
                ))}
              </div>
            )}
          </div>

          <div className="nav-tree-group">
            <button
              className={`nav-button nav-tree-toggle ${
                view === 'people' || selectedPerson ? 'active' : ''
              }`}
              onClick={() => {
                setPeopleExpanded((expanded) => !expanded);
                onNavigate('people');
              }}
              aria-expanded={peopleExpanded}
            >
              <UsersRound size={17} />
              <span>{t('people')}</span>
              <small className="nav-count">{library.people.length}</small>
              <ChevronRight
                size={14}
                className={`tree-chevron ${peopleExpanded ? 'expanded' : ''}`}
              />
            </button>

            {peopleExpanded && (
              <div className="tree-children">
                {library.people.map((person, index) => (
                  <button
                    key={person.id}
                    className={`tree-child person-child ${
                      selectedPerson?.id === person.id ? 'active' : ''
                    }`}
                    onClick={() => onPerson(person)}
                  >
                    <span>{locale === 'zh' ? person.nameZh || person.name : person.name}</span>
                    <small>{String(index + 1).padStart(2, '0')}</small>
                  </button>
                ))}
              </div>
            )}
          </div>

          <div className="nav-tree-group">
            <button
              className={`nav-button nav-tree-toggle ${
                view === 'stories' || selectedStory ? 'active' : ''
              }`}
              onClick={() => {
                setStoriesExpanded((expanded) => !expanded);
                onNavigate('stories');
              }}
              aria-expanded={storiesExpanded}
            >
              <BookOpen size={17} />
              <span>{t('stories')}</span>
              <small className="nav-count">{library.stories.length}</small>
              <ChevronRight
                size={14}
                className={`tree-chevron ${storiesExpanded ? 'expanded' : ''}`}
              />
            </button>

            {storiesExpanded && (
              <div className="tree-children">
                {library.stories.map((story, index) => (
                  <button
                    key={story.id}
                    className={`tree-child story-child ${
                      selectedStory?.id === story.id ? 'active' : ''
                    }`}
                    onClick={() => onStory(story)}
                  >
                    <span>{locale === 'zh' ? story.title : story.titleEn || story.title}</span>
                    <small>{String(index + 1).padStart(2, '0')}</small>
                  </button>
                ))}
              </div>
            )}
          </div>
        </nav>
      </div>
    </aside>
  );
}

function SidebarButton({
  icon,
  label,
  active,
  onClick,
  trailing,
}: {
  icon: ReactNode;
  label: string;
  active?: boolean;
  onClick: () => void;
  trailing?: ReactNode;
}) {
  return (
    <button className={`nav-button ${active ? 'active' : ''}`} onClick={onClick}>
      {icon}
      <span>{label}</span>
      {trailing}
    </button>
  );
}

function HomeView({
  locale,
  library,
  onCapture,
  onPeople,
  onPlan,
  onSupplement,
  t,
}: {
  locale: Locale;
  library: LibrarySnapshot;
  onCapture: () => void;
  onPeople: () => void;
  onPlan: () => void;
  onSupplement: (supplement: Supplement) => void;
  t: (key: TranslationKey) => string;
}) {
  const tiered = useMemo(() => {
    const tiers = ['T1', 'T2', 'T3', 'T4', 'T5'];
    return tiers.map((tier) => ({
      tier,
      supplements: library.supplements.filter((supplement) => supplement.tier === tier),
    }));
  }, [library.supplements]);
  const ambientAssignments = useMemo(() => createAmbientAssignments(3), []);

  return (
    <div className="home-view page">
      <section className="hero">
        <div className="hero-kicker">
          <Leaf size={15} />
          OPEN LONGEVITY
        </div>
        <h1>{t('greeting')}</h1>
      </section>

      <section className="start-section" aria-label={t('coreModules')}>
        <div className="start-cards">
          <ActionCard
            title={t('collectCard')}
            description={t('collectCardSub')}
            onClick={onCapture}
            ambient={ambientAssignments[0]}
          />
          <ActionCard
            title={t('peopleCard')}
            description={t('peopleCardSub')}
            onClick={onPeople}
            ambient={ambientAssignments[1]}
          />
          <ActionCard
            title={t('aiPlanCard')}
            description={t('aiPlanCardSub')}
            onClick={onPlan}
            ambient={ambientAssignments[2]}
          />
        </div>
      </section>

      <section className="tier-section" aria-label={t('evidenceMap')}>
        <div className="section-heading">
          <div>
            <p>{t('evidenceMapSub')}</p>
          </div>
          <span className="section-stat">{library.supplements.length} items</span>
        </div>
        <div className="tier-map">
          {tiered.map(({ tier, supplements }) => (
            <div className="tier-row" key={tier}>
              <div
                className="tier-label"
                style={{ '--tier-color': tierMeta[tier]?.color } as React.CSSProperties}
              >
                <strong>{tier}</strong>
              </div>
              <div className="tier-items">
                {supplements.length ? (
                  supplements.map((supplement) => (
                    <button key={supplement.id} onClick={() => onSupplement(supplement)}>
                      <span>{locale === 'zh' ? supplement.nameZh : supplement.nameEn}</span>
                    </button>
                  ))
                ) : (
                  <span className="tier-empty">—</span>
                )}
              </div>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}

function ActionCard({
  title,
  description,
  onClick,
  ambient,
}: {
  title: string;
  description: string;
  onClick: () => void;
  ambient: ReturnType<typeof createAmbientAssignments>[number];
}) {
  return (
    <button
      className="action-card"
      onClick={onClick}
      style={
        {
          '--ambient-delay': ambient.delay,
          '--ambient-duration': ambient.duration,
          '--ambient-direction': ambient.direction,
          '--ambient-secondary-delay': ambient.secondaryDelay,
          '--ambient-secondary-duration': ambient.secondaryDuration,
          '--ambient-secondary-direction': ambient.secondaryDirection,
        } as React.CSSProperties
      }
    >
      <span className="action-copy">
        <strong>{title}</strong>
        <small>{description}</small>
      </span>
      <ChevronRight size={18} />
    </button>
  );
}

function PeopleView({
  locale,
  people,
  onPerson,
  onBack,
  t,
}: {
  locale: Locale;
  people: Person[];
  onPerson: (person: Person) => void;
  onBack: () => void;
  t: (key: TranslationKey) => string;
}) {
  const ambientAssignments = useMemo(
    () => createAmbientAssignments(people.length),
    [people.length],
  );

  return (
    <div className="page people-view">
      <section className="page-intro">
        <div className="page-kicker-row">
          <PageBackButton locale={locale} onBack={onBack} />
          <div className="hero-kicker">
            <UsersRound size={15} />
            PROTOCOL ATLAS
          </div>
        </div>
        <h1>{t('peopleTitle')}</h1>
        <p>{t('peopleSub')}</p>
      </section>
      <div className="people-grid">
        {people.map((person, index) => (
          <button
            className="person-card"
            key={person.id}
            onClick={() => onPerson(person)}
            style={
              {
                '--person-accent': person.accent,
                '--ambient-delay': ambientAssignments[index].delay,
                '--ambient-duration': ambientAssignments[index].duration,
                '--ambient-direction': ambientAssignments[index].direction,
                '--ambient-secondary-delay': ambientAssignments[index].secondaryDelay,
                '--ambient-secondary-duration': ambientAssignments[index].secondaryDuration,
                '--ambient-secondary-direction': ambientAssignments[index].secondaryDirection,
              } as React.CSSProperties
            }
          >
            <span className="person-index">{String(index + 1).padStart(2, '0')}</span>
            <span className="person-avatar">
              <UserRound size={26} />
            </span>
            <span className="person-copy">
              <strong>{person.name}</strong>
              {person.nameZh && locale === 'zh' && <small className="person-zh">{person.nameZh}</small>}
              <small>{person.summary}</small>
            </span>
            <ArrowRight size={17} />
          </button>
        ))}
      </div>
    </div>
  );
}

function StoriesView({
  locale,
  stories,
  onStory,
  onAdd,
  onBack,
  t,
}: {
  locale: Locale;
  stories: Story[];
  onStory: (story: Story) => void;
  onAdd: () => void;
  onBack: () => void;
  t: (key: TranslationKey) => string;
}) {
  const ambientAssignments = useMemo(
    () => createAmbientAssignments(stories.length),
    [stories.length],
  );

  return (
    <div className="page stories-view">
      <section className="page-intro">
        <div className="page-kicker-row">
          <PageBackButton locale={locale} onBack={onBack} />
          <div className="hero-kicker">
            <BookOpen size={15} />
            LONGEVITY FIELD NOTES
          </div>
        </div>
        <h1>{t('storiesTitle')}</h1>
        <p>{t('storiesSub')}</p>
      </section>

      <div className="story-grid">
        {stories.map((story, index) => (
          <button
            className="story-card"
            key={story.id}
            onClick={() => onStory(story)}
            style={
              {
                '--story-accent': story.accent,
                '--ambient-delay': ambientAssignments[index].delay,
                '--ambient-duration': ambientAssignments[index].duration,
                '--ambient-direction': ambientAssignments[index].direction,
                '--ambient-secondary-delay': ambientAssignments[index].secondaryDelay,
                '--ambient-secondary-duration': ambientAssignments[index].secondaryDuration,
                '--ambient-secondary-direction': ambientAssignments[index].secondaryDirection,
              } as React.CSSProperties
            }
          >
            <span className="story-index">{String(index + 1).padStart(2, '0')}</span>
            <span className="story-copy">
              <small>FIELD NOTE</small>
              <strong>{locale === 'zh' ? story.title : story.titleEn || story.title}</strong>
              <span>
                {locale === 'zh' ? story.summary : story.summaryEn || story.summary}
              </span>
            </span>
            <ArrowRight size={18} />
          </button>
        ))}

        <button className="story-add-card" onClick={onAdd}>
          <span className="story-add-icon">
            <FilePlus2 size={22} />
          </span>
          <span>
            <strong>{locale === 'zh' ? '添加一则轶事' : 'Add a story'}</strong>
            <small>
              {locale === 'zh'
                ? '把文章或链接交给 AI，整理后加入资料库'
                : 'Give an article or link to AI and organize it into the library'}
            </small>
          </span>
        </button>
      </div>

      <div className="story-folder-note">
        <FolderOpen size={15} />
        <span>
          {locale === 'zh'
            ? '用户添加的 Markdown 放入 stories 目录后会自动成为新条目。'
            : 'Markdown files added to the stories folder are discovered automatically.'}
        </span>
      </div>
    </div>
  );
}

function PageBackButton({ locale, onBack }: { locale: Locale; onBack: () => void }) {
  return (
    <button className="page-back" onClick={onBack} aria-label={locale === 'zh' ? '返回' : 'Back'}>
      <ChevronRight className="back-chevron" size={17} />
    </button>
  );
}

function NoteView({
  eyebrow,
  title,
  tier,
  markdown,
  loading,
  locale,
  currentTarget,
  internalTargets,
  onInternalNavigate,
  favorite,
  onToggleFavorite,
  onBack,
}: {
  eyebrow: string;
  title: string;
  tier?: string;
  markdown: string;
  loading: boolean;
  locale: Locale;
  currentTarget: Omit<InternalNoteTarget, 'label'>;
  internalTargets: InternalNoteTarget[];
  onInternalNavigate: (target: Omit<InternalNoteTarget, 'label'>) => void;
  favorite: boolean;
  onToggleFavorite: () => void;
  onBack: () => void;
}) {
  const renderedMarkdown = useMemo(
    () =>
      linkInternalKeywords(
        normalizeMarkdown(markdown),
        internalTargets,
        currentTarget,
      ),
    [currentTarget.id, currentTarget.kind, internalTargets, markdown],
  );
  const components = useMemo(
    () => ({
      a: (
        props: React.AnchorHTMLAttributes<HTMLAnchorElement> & {
          node?: unknown;
        },
      ) => <AppLink {...props} onInternalNavigate={onInternalNavigate} />,
    }),
    [onInternalNavigate],
  );

  return (
    <article className="page note-view">
      <div className="note-header">
        <div>
          <div className="note-eyebrow-row">
            <PageBackButton locale={locale} onBack={onBack} />
            <span className="note-eyebrow">{eyebrow}</span>
            <button
              type="button"
              className={`note-favorite ${favorite ? 'active' : ''}`}
              aria-label={translate(locale, favorite ? 'removeFavorite' : 'addFavorite')}
              aria-pressed={favorite}
              onClick={onToggleFavorite}
            >
              <Star size={18} fill={favorite ? 'currentColor' : 'none'} />
            </button>
          </div>
          <h1>{title}</h1>
        </div>
        {tier && (
          <span
            className="large-tier"
            style={{ '--tier-color': tierMeta[tier]?.color || '#bdc9d3' } as React.CSSProperties}
          >
            {tier}
          </span>
        )}
      </div>
      {loading ? (
        <div className="loading-state compact">
          <LoaderCircle className="spin" size={22} />
        </div>
      ) : (
        <div className="markdown-body">
          <ReactMarkdown remarkPlugins={[remarkGfm]} components={components}>
            {renderedMarkdown}
          </ReactMarkdown>
        </div>
      )}
    </article>
  );
}

function ConversationView({
  locale,
  messages,
  busy,
  onBack,
  onNewChat,
  onInternalNavigate,
}: {
  locale: Locale;
  messages: ChatMessage[];
  busy: boolean;
  onBack: () => void;
  onNewChat: () => void;
  onInternalNavigate: (target: Omit<InternalNoteTarget, 'label'>) => void;
}) {
  const endRef = useRef<HTMLDivElement>(null);
  const components = useMemo(
    () => ({
      a: (
        props: React.AnchorHTMLAttributes<HTMLAnchorElement> & {
          node?: unknown;
        },
      ) => <AppLink {...props} onInternalNavigate={onInternalNavigate} />,
    }),
    [onInternalNavigate],
  );
  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages, busy]);

  return (
    <div className="page conversation-view">
      <div className="conversation-title">
        <button onClick={onBack}>
          <ChevronRight size={15} className="back-chevron" />
          {locale === 'zh' ? '返回资料' : 'Back to library'}
        </button>
        <div className="conversation-title-actions">
          <button className="conversation-new-chat" onClick={onNewChat} title={locale === 'zh' ? '新对话' : 'New chat'}>
            <FilePlus2 size={14} />
            <span>{locale === 'zh' ? '新对话' : 'New chat'}</span>
          </button>
          <span>{locale === 'zh' ? '基于本地资料' : 'Grounded in local notes'}</span>
        </div>
      </div>
      <div className="message-list">
        {messages.map((message) => {
          if (message.role === 'tool_call') {
            const toolLabels: Record<string, string> = {
              save_note: locale === 'zh' ? '保存笔记' : 'Save note',
              search_library: locale === 'zh' ? '搜索知识库' : 'Search library',
              read_note: locale === 'zh' ? '读取笔记' : 'Read note',
            };
            const label = toolLabels[message.toolName || ''] || message.toolName || 'tool';
            const statusIcon =
              message.toolStatus === 'running' ? (
                <LoaderCircle size={12} className="spin" />
              ) : message.toolStatus === 'failed' ? (
                <X size={12} />
              ) : (
                <Check size={12} />
              );
            return (
              <div className="tool-call-card" key={message.id}>
                <button
                  type="button"
                  className="tool-call-header"
                  onClick={(e) => {
                    const card = (e.currentTarget as HTMLElement).closest('.tool-call-card');
                    card?.classList.toggle('open');
                  }}
                >
                  <Wrench size={13} />
                  <span className="tool-call-label">{label}</span>
                  <span className="tool-call-status">{statusIcon}</span>
                </button>
                <div className="tool-call-body">
                  {message.toolArgs && (
                    <pre className="tool-call-args">{message.toolArgs}</pre>
                  )}
                  {message.toolOutput && (
                    <pre className={`tool-call-output ${message.toolStatus === 'failed' ? 'failed' : ''}`}>
                      {message.toolOutput.length > 2000
                        ? message.toolOutput.slice(0, 2000) + '\n…'
                        : message.toolOutput}
                    </pre>
                  )}
                </div>
              </div>
            );
          }
          return (
            <div className={`message ${message.role}`} key={message.id}>
              <div className="message-avatar">
                {message.role === 'assistant' ? <Leaf size={17} /> : <CircleUserRound size={18} />}
              </div>
              <div className="message-content">
                <span className="message-author">
                  {message.role === 'assistant'
                    ? 'Open Longevity'
                    : locale === 'zh'
                      ? '你'
                      : 'You'}
                </span>
                <ReactMarkdown remarkPlugins={[remarkGfm]} components={components}>
                  {message.content}
                </ReactMarkdown>
              </div>
            </div>
          );
        })}
        {busy && (
          <div className="message assistant">
            <div className="message-avatar">
              <Leaf size={17} />
            </div>
            <div className="message-content thinking-line">
              <span />
              <span />
              <span />
            </div>
          </div>
        )}
        <div ref={endRef} />
      </div>
    </div>
  );
}

function HealthLogPanel({ locale }: { locale: Locale }) {
  const [log, setLog] = useStoredState<HealthLog>(HEALTH_LOG_KEY, {});
  const [date, setDate] = useState<string>(() => todayKey());
  const entry = log[date] || {};
  const isToday = date === todayKey();

  const setField = (field: HealthLogField, value: string) => {
    setLog({ ...log, [date]: { ...entry, [field]: value } });
  };

  const clearDay = () => {
    const next = { ...log };
    delete next[date];
    setLog(next);
  };

  const recordedCount = Object.keys(log).filter((key) => entryHasContent(log[key])).length;
  const weekKeys = Array.from({ length: 7 }, (_, index) => shiftKey(todayKey(), index - 6));
  const weekdays =
    locale === 'zh' ? ['日', '一', '二', '三', '四', '五', '六'] : ['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa'];

  const fields: Array<{ id: HealthLogField; label: string; placeholder: string; icon: ReactNode }> = [
    {
      id: 'exercise',
      label: locale === 'zh' ? '有氧&力量' : 'Cardio & Strength',
      placeholder:
        locale === 'zh' ? '例如：快走 30 分钟、力量训练、拉伸' : 'e.g. 30 min walk, strength training, stretching',
      icon: <Dumbbell size={16} />,
    },
    {
      id: 'diet',
      label: locale === 'zh' ? '饮食&补剂' : 'Food & Supplements',
      placeholder:
        locale === 'zh' ? '例如：三餐内容、蛋白质、补剂、进食时间' : 'e.g. meals, protein, supplements, meal timing',
      icon: <Utensils size={16} />,
    },
    {
      id: 'body',
      label: locale === 'zh' ? '数据&记录' : 'Data & Notes',
      placeholder:
        locale === 'zh' ? '例如：体重、睡眠时长、血压、当日感受' : 'e.g. weight, sleep hours, blood pressure, how you feel',
      icon: <Activity size={16} />,
    },
  ];

  const recordedText =
    locale === 'zh'
      ? '已记录 ' + recordedCount + ' 天'
      : recordedCount + ' day' + (recordedCount === 1 ? '' : 's') + ' logged';

  return (
    <div className="health-log">
      <div className="health-log-head">
        <div>
          <span className="health-log-kicker">
            <NotebookPen size={15} />
            {locale === 'zh' ? '每日记录' : 'DAILY LOG'}
          </span>
          <strong>{locale === 'zh' ? '健康记录' : 'Health log'}</strong>
          <small>{recordedText}</small>
        </div>
      </div>

      <div className="health-week" role="group" aria-label={locale === 'zh' ? '最近七天' : 'Last seven days'}>
        {weekKeys.map((key) => {
          const d = new Date(key + 'T00:00:00');
          const has = entryHasContent(log[key]);
          const active = key === date;
          const today = key === todayKey();
          return (
            <button
              type="button"
              key={key}
              className={
                'health-week-day' +
                (active ? ' active' : '') +
                (has ? ' has' : '') +
                (today ? ' today' : '')
              }
              onClick={() => setDate(key)}
            >
              <span className="health-week-wd">{weekdays[d.getDay()]}</span>
              <span className="health-week-num">{d.getDate()}</span>
              <span className="health-week-dot" />
            </button>
          );
        })}
      </div>

      <div className="health-datebar">
        <button
          type="button"
          className="health-date-nav"
          onClick={() => setDate(shiftKey(date, -1))}
          aria-label={locale === 'zh' ? '前一天' : 'Previous day'}
        >
          <ChevronLeft size={18} />
        </button>
        <input
          type="date"
          className="health-date-input"
          value={date}
          onChange={(event) => setDate(event.target.value || date)}
          aria-label={locale === 'zh' ? '选择日期' : 'Pick a date'}
        />
        <button
          type="button"
          className="health-date-nav"
          onClick={() => setDate(shiftKey(date, 1))}
          aria-label={locale === 'zh' ? '后一天' : 'Next day'}
        >
          <ChevronRight size={18} />
        </button>
        <button
          type="button"
          className={'health-today' + (isToday ? ' active' : '')}
          onClick={() => setDate(todayKey())}
        >
          {locale === 'zh' ? '今天' : 'Today'}
        </button>
      </div>

      <div className="health-fields">
        {fields.map((field) => (
          <label className="health-field" key={field.id}>
            <span className="health-field-label">
              <span className="health-field-icon">{field.icon}</span>
              {field.label}
            </span>
            <textarea
              rows={3}
              value={entry[field.id] || ''}
              placeholder={field.placeholder}
              onChange={(event) => setField(field.id, event.target.value)}
            />
          </label>
        ))}
      </div>

      <div className="health-log-foot">
        <span className="health-saved">
          <Check size={15} />
          {locale === 'zh' ? '自动保存到本机' : 'Saved locally as you type'}
        </span>
        {entryHasContent(entry) ? (
          <button type="button" className="health-clear" onClick={clearDay}>
            <Trash2 size={15} />
            {locale === 'zh' ? '清空当天' : 'Clear day'}
          </button>
        ) : null}
      </div>
    </div>
  );
}

function PlanView({
  locale,
  activeSection,
  onSection,
  onBack,
  t,
}: {
  locale: Locale;
  activeSection: PlanSection;
  onSection: (section: PlanSection) => void;
  onBack: () => void;
  t: (key: TranslationKey) => string;
}) {
  const sections = getPlanSections(locale);
  const active = sections.find((section) => section.id === activeSection) || sections[0];

  return (
    <div className="page plan-view">
      <section className="page-intro">
        <div className="page-kicker-row">
          <PageBackButton locale={locale} onBack={onBack} />
          <div className="hero-kicker">
            <Sparkles size={15} />
            PERSONAL PROTOCOL
          </div>
        </div>
        <h1>{t('planTitle')}</h1>
        <p>{t('planSub')}</p>
      </section>

      <div className="plan-section-grid">
        {sections.map((section) => (
          <button
            className={section.id === activeSection ? 'active' : ''}
            onClick={() => onSection(section.id)}
            key={section.id}
          >
            <span className="plan-section-icon" style={{ background: section.accent }}>
              {section.icon}
            </span>
            <span>
              <strong>{section.title}</strong>
              <small>{section.description}</small>
            </span>
          </button>
        ))}
      </div>

      {activeSection === 'log' ? (
        <HealthLogPanel locale={locale} />
      ) : (
      <div className="plan-focus">
        <span className="plan-focus-icon" style={{ background: active.accent }}>
          {active.icon}
        </span>
        <div>
          <small>{locale === 'zh' ? '当前计划' : 'Current plan'}</small>
          <strong>{active.title}</strong>
          <p>
            {locale === 'zh'
              ? '在下方对话中告诉 AI 你的目标、现状和限制，它会优先结合本地个人方案帮你整理。'
              : 'Tell AI your goals, current status, and constraints below. It will prioritize your local personal protocol.'}
          </p>
          <span className="plan-file">plans/current-protocol.md</span>
        </div>
      </div>
      )}
    </div>
  );
}

function ChatComposer({
  busy,
  onSend,
  placeholder,
  inputRef,
}: {
  busy: boolean;
  onSend: (message: string) => void;
  placeholder: string;
  inputRef: React.RefObject<HTMLTextAreaElement>;
}) {
  const [value, setValue] = useState('');

  const submit = (event: FormEvent) => {
    event.preventDefault();
    if (!value.trim()) return;
    onSend(value);
    setValue('');
  };

  return (
    <div className="composer-wrap">
      <form className="composer" onSubmit={submit}>
        <textarea
          ref={inputRef}
          rows={1}
          value={value}
          placeholder={placeholder}
          onChange={(event) => setValue(event.target.value)}
          onKeyDown={(event) => {
            if (event.key === 'Enter' && !event.shiftKey) {
              event.preventDefault();
              event.currentTarget.form?.requestSubmit();
            }
          }}
          aria-label={placeholder}
        />
        <div className="composer-tools">
          <button type="submit" disabled={busy || !value.trim()}>
            {busy ? <LoaderCircle className="spin" size={17} /> : <Send size={17} />}
          </button>
        </div>
      </form>
    </div>
  );
}

function RightRail({
  locale,
  view,
  chatActive,
  messages,
  supplement,
  person,
  story,
  references,
  library,
  favorites,
  activePlanSection,
  onFavoriteNavigate,
  onPlanSection,
  onResumeChat,
  t,
}: {
  locale: Locale;
  view: View;
  chatActive: boolean;
  messages: ChatMessage[];
  supplement: Supplement | null;
  person: Person | null;
  story: Story | null;
  references: Array<{ label: string; url: string }>;
  library: LibrarySnapshot;
  favorites: FavoriteReference[];
  activePlanSection: PlanSection;
  onFavoriteNavigate: (target: Omit<InternalNoteTarget, 'label'>) => void;
  onPlanSection: (section: PlanSection) => void;
  onResumeChat: () => void;
  t: (key: TranslationKey) => string;
}) {
  const planSections = getPlanSections(locale);
  const hasOpenContent = Boolean(supplement || person || story);
  const favoriteItems = useMemo<FavoriteListItem[]>(() => {
    const items: FavoriteListItem[] = [];
    for (const favorite of favorites) {
      if (favorite.kind === 'supplement') {
        const item = library.supplements.find((candidate) => candidate.id === favorite.id);
        if (item) {
          items.push({
            target: favorite,
            title: locale === 'zh' ? item.nameZh : item.nameEn,
            detail: `${item.tier} · ${item.category}`,
          });
        }
      } else if (favorite.kind === 'person') {
        const item = library.people.find((candidate) => candidate.id === favorite.id);
        if (item) {
          items.push({
            target: favorite,
            title: locale === 'zh' ? item.nameZh || item.name : item.name,
            detail: t('people'),
          });
        }
      } else {
        const item = library.stories.find((candidate) => candidate.id === favorite.id);
        if (item) {
          items.push({
            target: favorite,
            title: locale === 'zh' ? item.title : item.titleEn || item.title,
            detail: t('stories'),
          });
        }
      }
    }
    return items;
  }, [favorites, library, locale, t]);

  return (
    <aside className="right-rail">
      <div className="rail-header">
        <div>
          <span className="rail-kicker">
            {chatActive ? (
              <History size={15} />
            ) : hasOpenContent ? (
              <Library size={15} />
            ) : (
              <Sparkles size={15} />
            )}
            {chatActive
              ? t('recentContexts')
              : hasOpenContent
                ? t('reading')
                : t('workspace')}
          </span>
          <h3>
            {chatActive
              ? locale === 'zh'
                ? '当前对话'
                : 'Current conversation'
              : (supplement
                  ? locale === 'zh'
                    ? supplement.nameZh
                    : supplement.nameEn
                  : null) ||
                (person
                  ? locale === 'zh'
                    ? person.nameZh || person.name
                    : person.name
                  : null) ||
                (story ? (locale === 'zh' ? story.title : story.titleEn || story.title) : null) ||
                t('favoritesAndPlan')}
          </h3>
        </div>
        {!chatActive && messages.length > 0 && (
          <button type="button" className="rail-resume-chat" onClick={onResumeChat}>
            <MessageCircleMore size={14} />
            {t('backToChat')}
          </button>
        )}
      </div>

      <div className="rail-scroll">
        {chatActive ? (
          <>
            <div className="context-summary">
              <span className="context-icon">
                <BookOpen size={18} />
              </span>
              <div>
                <strong>
                  {library.noteCount} {locale === 'zh' ? '篇本地笔记' : 'local notes'}
                </strong>
                <small>
                  {locale === 'zh'
                    ? '个人方案与当前页面会优先进入上下文'
                    : 'Personal protocol and the open note are prioritized'}
                </small>
              </div>
            </div>
            <div className="rail-section-title">
              {locale === 'zh' ? '对话记录' : 'Messages'} <span>{messages.length}</span>
            </div>
            <div className="rail-message-list">
              {messages
                .filter((message) => message.role === 'user')
                .slice(-5)
                .reverse()
                .map((message) => (
                  <div className="rail-message" key={message.id}>
                    <MessageCircleMore size={15} />
                    <span>{message.content}</span>
                  </div>
                ))}
            </div>
          </>
        ) : (
          <>
            <div className="rail-section-title">
              {t('favorites')} {favoriteItems.length ? <span>{favoriteItems.length}</span> : null}
            </div>
            {favoriteItems.length ? (
              <div className="favorite-list">
                {favoriteItems.map((item) => (
                  <button
                    type="button"
                    onClick={() => onFavoriteNavigate(item.target)}
                    key={`${item.target.kind}:${item.target.id}`}
                  >
                    <span className="favorite-item-icon">
                      {item.target.kind === 'supplement' ? (
                        <Dumbbell size={17} />
                      ) : item.target.kind === 'person' ? (
                        <UserRound size={17} />
                      ) : (
                        <BookOpen size={17} />
                      )}
                    </span>
                    <span>
                      <strong>{item.title}</strong>
                      <small>{item.detail}</small>
                    </span>
                    <ChevronRight size={15} />
                  </button>
                ))}
              </div>
            ) : (
              <p className="favorite-inline-empty">{t('favoriteHint')}</p>
            )}

            <div className="rail-section-title">
              {t('myPlan')} <span>{planSections.length}</span>
            </div>
            <div className="plan-shortcut-list">
              {planSections.map((section) => (
                <button
                  className={view === 'plan' && activePlanSection === section.id ? 'active' : ''}
                  onClick={() => onPlanSection(section.id)}
                  key={section.id}
                >
                  <span className="plan-shortcut-icon" style={{ background: section.accent }}>
                    {section.icon}
                  </span>
                  <span>
                    <strong>{section.title}</strong>
                    <small>{section.description}</small>
                  </span>
                  <ChevronRight size={15} />
                </button>
              ))}
            </div>

            {references.length > 0 && (
              <>
                <div className="rail-section-title">
                  {t('sources')} <span>{references.length}</span>
                </div>
                <div className="source-list">
                  {references.map((reference) => (
                    <AppLink href={reference.url} key={reference.url}>
                      <Globe2 size={15} />
                      <span>{reference.label}</span>
                      <ArrowRight size={14} />
                    </AppLink>
                  ))}
                </div>
              </>
            )}
          </>
        )}
      </div>

      <div className="rail-disclosure">
        <ShieldCheck size={15} />
        <span>{t('disclosure')}</span>
      </div>
    </aside>
  );
}

function SettingsDialog({
  locale,
  config,
  knowledgeRoot,
  themeMode,
  onChange,
  onLocale,
  onThemeMode,
  onChooseFolder,
  onClose,
  t,
}: {
  locale: Locale;
  config: ModelConfig;
  knowledgeRoot: string;
  themeMode: ThemeMode;
  onChange: (config: ModelConfig) => void;
  onLocale: (locale: Locale) => void;
  onThemeMode: (themeMode: ThemeMode) => void;
  onChooseFolder: () => void;
  onClose: () => void;
  t: (key: TranslationKey) => string;
}) {
  const [draft, setDraft] = useState(config);

  const updateProvider = (provider: ModelConfig['provider']) => {
    const preset = providerPresets[provider];
    const next = {
      ...draft,
      provider,
      baseUrl: preset.baseUrl,
      model: preset.model,
    };
    setDraft(next);
    onChange(next);
  };

  const updateDraft = (patch: Partial<ModelConfig>) => {
    const next = { ...draft, ...patch };
    setDraft(next);
    onChange(next);
  };

  return (
    <div className="modal-backdrop" onMouseDown={onClose}>
      <section className="settings-dialog" onMouseDown={(event) => event.stopPropagation()}>
        <header className="dialog-header">
          <div>
            <span className="dialog-icon">
              <Settings size={19} />
            </span>
            <div>
              <h2>{t('modelSettings')}</h2>
            </div>
          </div>
          <button onClick={onClose} aria-label="Close">
            <X size={19} />
          </button>
        </header>

        <div className="settings-content">
          <div className="settings-section">
            <label>{t('provider')}</label>
            <div className="provider-grid">
              {(Object.keys(providerPresets) as ModelConfig['provider'][]).map((provider) => (
                <button
                  className={draft.provider === provider ? 'active' : ''}
                  onClick={() => updateProvider(provider)}
                  key={provider}
                >
                  {providerPresets[provider].label[locale]}
                  {draft.provider === provider && <Check size={14} />}
                </button>
              ))}
            </div>
            <div className="field-row">
              <label>
                <span>{t('baseUrl')}</span>
                <input
                  value={draft.baseUrl}
                  onChange={(event) => updateDraft({ baseUrl: event.target.value })}
                  placeholder="e.g. https://api.example.com/v1"
                />
              </label>
              <label>
                <span>{t('model')}</span>
                <input
                  value={draft.model}
                  onChange={(event) => updateDraft({ model: event.target.value })}
                  placeholder="e.g. model-id"
                />
              </label>
            </div>
            <label className="full-field">
              <span>{t('apiKey')}</span>
              <input
                type="password"
                value={draft.apiKey}
                onChange={(event) => updateDraft({ apiKey: event.target.value })}
                placeholder="e.g. sk-…"
                autoComplete="off"
              />
            </label>
          </div>

          <div className="settings-section">
            <label>{t('knowledgeRoot')}</label>
            <button className="folder-picker" onClick={onChooseFolder}>
              <span>
                <FolderOpen size={17} />
                <strong>{knowledgeRoot || '—'}</strong>
              </span>
              <span>{t('chooseFolder')}</span>
            </button>
          </div>

          <div className="settings-section">
            <label>{t('appearance')}</label>
            <div className="theme-switch">
              <button
                className={themeMode === 'system' ? 'active' : ''}
                onClick={() => onThemeMode('system')}
              >
                <Monitor size={15} />
                {t('themeSystem')}
              </button>
              <button
                className={themeMode === 'light' ? 'active' : ''}
                onClick={() => onThemeMode('light')}
              >
                <Sun size={15} />
                {t('themeLight')}
              </button>
              <button
                className={themeMode === 'dark' ? 'active' : ''}
                onClick={() => onThemeMode('dark')}
              >
                <Moon size={15} />
                {t('themeDark')}
              </button>
            </div>
          </div>

          <div className="settings-section">
            <label>{t('language')}</label>
            <div className="language-switch">
              <button className={locale === 'zh' ? 'active' : ''} onClick={() => onLocale('zh')}>
                中文
              </button>
              <button className={locale === 'en' ? 'active' : ''} onClick={() => onLocale('en')}>
                English
              </button>
            </div>
          </div>
        </div>

        <footer className="dialog-footer">
          <div className="dialog-meta">
            <span>Open Longevity · v{APP_VERSION}</span>
            <button onClick={() => void openExternalUrl(FEEDBACK_URL)}>
              <MessageCircleMore size={13} strokeWidth={1.8} />
              {t('feedback')}
            </button>
          </div>
          <button className="primary-button" onClick={onClose}>
            {t('close')}
          </button>
        </footer>
      </section>
    </div>
  );
}

function CaptureGuideDialog({
  locale,
  config,
  knowledgeRoot,
  onClose,
  onSaved,
  t,
}: {
  locale: Locale;
  config: ModelConfig;
  knowledgeRoot: string;
  onClose: () => void;
  onSaved: (path: string) => Promise<void>;
  t: (key: TranslationKey) => string;
}) {
  const [source, setSource] = useState('');
  const [draft, setDraft] = useState<CaptureDraft | null>(null);
  const [busy, setBusy] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');

  const organize = async () => {
    const clean = source.trim();
    if (!clean) {
      setError(t('captureInputRequired'));
      return;
    }
    if (isTauri && !config.apiKey.trim()) {
      setError(t('captureNeedsModel'));
      return;
    }

    setBusy(true);
    setError('');
    try {
      setDraft(
        await prepareCapture({
          apiKey: config.apiKey,
          baseUrl: config.baseUrl,
          model: config.model,
          input: clean,
          locale,
        }),
      );
    } catch (requestError) {
      setError(
        `${t('capturePrepareFailed')}: ${String(requestError).replace(/^Error:\s*/i, '')}`,
      );
    } finally {
      setBusy(false);
    }
  };

  const save = async () => {
    if (!draft || saving) return;
    if (!knowledgeRoot) {
      setError(t('captureNeedsLibrary'));
      return;
    }
    if (!draft.title.trim() || !draft.content.trim()) {
      setError(t('captureDraftRequired'));
      return;
    }

    setSaving(true);
    setError('');
    try {
      const path = await saveCapture({
        knowledgeRoot,
        title: draft.title,
        content: draft.content,
        sourceUrl: draft.sourceUrl,
        locale,
      });
      await onSaved(path);
    } catch (saveError) {
      setError(`${t('captureSaveFailed')}: ${String(saveError).replace(/^Error:\s*/i, '')}`);
      setSaving(false);
    }
  };

  return (
    <div className="modal-backdrop capture-backdrop" onMouseDown={onClose}>
      <section className="capture-dialog" onMouseDown={(event) => event.stopPropagation()}>
        <header className="dialog-header">
          <div>
            <span className="dialog-icon capture-icon">
              <Bot size={19} />
            </span>
            <div>
              <h2>{t('captureTitle')}</h2>
              <p>{t('captureSub')}</p>
            </div>
          </div>
          <button onClick={onClose} aria-label="Close">
            <X size={19} />
          </button>
        </header>

        <div className="capture-guide">
          {!draft ? (
            <>
              <label className="capture-field">
                <span>{t('captureInputLabel')}</span>
                <textarea
                  value={source}
                  onChange={(event) => setSource(event.target.value)}
                  placeholder={t('captureInputPlaceholder')}
                  maxLength={180000}
                  autoFocus
                />
              </label>
              <div className="capture-prompt-example">
                <Sparkles size={17} />
                <div>
                  <strong>{t('captureExampleTitle')}</strong>
                  <p>{t('captureExample')}</p>
                </div>
              </div>
            </>
          ) : (
            <div className="capture-draft">
              <label className="capture-field">
                <span>{t('captureDraftTitle')}</span>
                <input
                  value={draft.title}
                  onChange={(event) => setDraft({ ...draft, title: event.target.value })}
                  maxLength={180}
                />
              </label>
              <label className="capture-field">
                <span>{t('captureDraftContent')}</span>
                <textarea
                  className="capture-draft-content"
                  value={draft.content}
                  onChange={(event) => setDraft({ ...draft, content: event.target.value })}
                  maxLength={120000}
                />
              </label>
              {draft.sourceUrl && <p className="capture-source-url">{draft.sourceUrl}</p>}
            </div>
          )}

          {error && (
            <p className="capture-error" role="alert">
              {error}
            </p>
          )}

          <div className="capture-guide-actions">
            <button
              className="secondary-button"
              onClick={draft ? () => setDraft(null) : onClose}
              disabled={busy || saving}
            >
              {draft ? t('captureBack') : t('notNow')}
            </button>
            <button
              className="primary-button"
              onClick={draft ? save : organize}
              disabled={busy || saving || (!draft && !source.trim())}
            >
              {busy || saving ? (
                <LoaderCircle className="spinning" size={16} />
              ) : draft ? (
                <Check size={16} />
              ) : (
                <Sparkles size={16} />
              )}
              {busy
                ? t('capturePreparing')
                : saving
                  ? t('captureSaving')
                  : draft
                    ? t('captureConfirmSave')
                    : t('capturePrepare')}
            </button>
          </div>
          <p className="capture-guide-note">{t('captureNote')}</p>
        </div>
      </section>
    </div>
  );
}

export default App;
