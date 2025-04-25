<template>
  <div>
    <Select :model-value="locale.value" @update:model-value="changeLocale">
      <SelectTrigger>
        <SelectValue :placeholder="t('language.select')" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectItem v-for="locale in SUPPORT_LOCALES" :key="locale" :value="locale">
            {{ getLocaleName(locale) }}
          </SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { SUPPORT_LOCALES, type SupportLocale, setLocale } from '../i18n';
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from './ui/select';

const { t, locale } = useI18n();

const getLocaleName = (locale: SupportLocale) => {
  switch (locale) {
    case 'en':
      return 'English';
    case 'zh-CN':
      return '中文';
    default:
      return locale;
  }
};

const changeLocale = (newLocale: SupportLocale) => {
  setLocale(newLocale);
};
</script>