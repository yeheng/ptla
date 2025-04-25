<script setup lang="ts">
import { RouterView } from 'vue-router';
</script>

<template>
  <div class="min-h-screen bg-background font-sans antialiased">
    <div class="relative flex min-h-screen flex-col">
      <div class="flex-1">
        <div class="container mx-auto p-4">
          <router-view></router-view>
        </div>
      </div>
    </div>
  </div>
</template>