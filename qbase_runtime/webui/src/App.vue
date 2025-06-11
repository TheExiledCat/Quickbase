<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
import { Toast } from "primevue";
</script>

<template>
  <Toast />
  <router-view v-slot="{ Component }">
    <transition name="fadeslide" mode="out-in">
      <component :is="Component" />
    </transition>
  </router-view>
</template>

<style>
.fadeslide-enter-active,
.fadeslide-leave-active {
  transition: transform 0.5s ease, opacity 0.5s ease;
}

.fadeslide-enter-from {
  transform: translateX(-100%);
  opacity: 0;
}

.fadeslide-enter-to {
  transform: translateX(0%);
  opacity: 1;
}

.fadeslide-leave-from {
  transform: translateX(0%);
  opacity: 1;
}

.fadeslide-leave-to {
  transform: translateX(100%);
  opacity: 0;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
