import { createI18n } from 'vue-i18n';
import en from './en.json';
import zhCN from './zh-CN.json';

export const SUPPORT_LOCALES = ['en', 'zh-CN'] as const;

export type SupportLocale = typeof SUPPORT_LOCALES[number];

// 从 localStorage 获取保存的语言设置，如果没有则使用浏览器语言
const savedLocale = localStorage.getItem('locale') as SupportLocale;
const browserLocale = navigator.language;
const defaultLocale = savedLocale || 
  (SUPPORT_LOCALES.includes(browserLocale as SupportLocale) ? browserLocale : 'zh-CN');

export const i18n = createI18n({
  legacy: false, // 使用 Composition API 模式
  locale: defaultLocale,
  fallbackLocale: 'en',
  messages: {
    'en': en,
    'zh-CN': zhCN,
  },
});

// 切换语言的函数
export const setLocale = (locale: SupportLocale) => {
  i18n.global.locale.value = locale;
  localStorage.setItem('locale', locale);
  document.querySelector('html')?.setAttribute('lang', locale);
}; 

export const getLocale = () => {
  return i18n.global.locale.value;
}; 