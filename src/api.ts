import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { openUrl } from '@tauri-apps/plugin-opener';
import { fallbackLibrary, fallbackMarkdown } from './data';
import type {
  CaptureDraft,
  CaptureRequest,
  ChatRequest,
  LibrarySnapshot,
  PrepareCaptureRequest,
} from './types';

export const isTauri = '__TAURI_INTERNALS__' in window;

export async function openExternalUrl(url: string): Promise<void> {
  const parsed = new URL(url);
  if (parsed.protocol !== 'http:' && parsed.protocol !== 'https:') {
    throw new Error(`Unsupported external URL protocol: ${parsed.protocol}`);
  }

  if (isTauri) {
    await openUrl(parsed.href);
    return;
  }

  window.open(parsed.href, '_blank', 'noopener,noreferrer');
}

export interface SelfUpdateProgress {
  status: 'checking' | 'downloading' | 'launching' | 'error';
  percent: number;
}

export async function checkForUpdate(): Promise<string | null> {
  if (!isTauri) return null;
  return invoke<string | null>('check_for_update');
}

export async function downloadAndInstallUpdate(): Promise<void> {
  if (!isTauri) {
    throw new Error('Self-update is only available in the desktop app.');
  }
  await invoke('download_and_install_update');
}

export async function onSelfUpdateProgress(
  callback: (progress: SelfUpdateProgress) => void,
): Promise<() => void> {
  const { listen } = await import('@tauri-apps/api/event');
  return listen<SelfUpdateProgress>('self-update-progress', (event) => callback(event.payload));
}

export async function loadLibrary(root: string | undefined, locale: 'zh' | 'en'): Promise<LibrarySnapshot> {
  if (!isTauri) {
    return { ...fallbackLibrary, root: root || fallbackLibrary.root };
  }
  return invoke<LibrarySnapshot>('load_library', { root: root || null, locale });
}

export async function readNote(root: string, relativePath: string): Promise<string> {
  if (!isTauri) {
    return (
      fallbackMarkdown[relativePath] ||
      `# ${relativePath.split('/').pop()?.replace('.md', '') || 'Note'}\n\n这篇笔记会从你的本地知识库读取。`
    );
  }
  return invoke<string>('read_note', { root, relativePath });
}

export async function chooseKnowledgeFolder(): Promise<string | null> {
  if (!isTauri) return null;
  const selected = await open({ directory: true, multiple: false });
  return typeof selected === 'string' ? selected : null;
}

export async function saveCapture(request: CaptureRequest): Promise<string> {
  if (!isTauri) {
    const captures = JSON.parse(localStorage.getItem('openlongevity:captures') || '[]');
    captures.push({ ...request, createdAt: Date.now() });
    localStorage.setItem('openlongevity:captures', JSON.stringify(captures));
    return 'prototype/inbox';
  }
  return invoke<string>('save_capture', { request });
}

export async function prepareCapture(request: PrepareCaptureRequest): Promise<CaptureDraft> {
  if (!isTauri) {
    await new Promise((resolve) => setTimeout(resolve, 650));
    const sourceUrl = /^https?:\/\/\S+$/i.test(request.input.trim())
      ? request.input.trim()
      : undefined;
    return {
      title: request.locale === 'zh' ? '待核查的科学笔记' : 'Science note for verification',
      content:
        request.locale === 'zh'
          ? `## 原始资料\n\n${request.input.trim()}\n\n## 待核查事项\n\n- 浏览器预览不会调用模型或抓取网页。`
          : `## Source material\n\n${request.input.trim()}\n\n## Items to verify\n\n- The browser preview does not call a model or fetch webpages.`,
      sourceUrl,
    };
  }
  return invoke<CaptureDraft>('prepare_capture', { request });
}

export async function chatCompletion(request: ChatRequest): Promise<string> {
  if (!isTauri) {
    await new Promise((resolve) => setTimeout(resolve, 650));
    return request.locale === 'zh'
      ? '当前浏览器预览已连接界面与本地示例数据。安装桌面版后，我会先检索你的个人方案与相关档案，再基于命中的笔记回答，并在右侧列出上下文。'
      : 'This browser preview is wired to sample local data. In the desktop app, I first retrieve your personal protocol and related dossiers, answer from that context, and show what was used on the right.';
  }
  return invoke<string>('chat_completion', { request });
}
