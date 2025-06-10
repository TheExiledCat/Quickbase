<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
import { Toast } from "primevue";
</script>

<template>
  <Toast />
  <router-view v-slot="{ Component }">
    <transition name="fade" mode="out-in">
      <component :is="Component" />
    </transition>
  </router-view>
</template>

<style>
.fade-enter-active,
.fade-leave-active {
  transition: transform 0.5s ease, opacity 0.5s ease;
}

.fade-enter-from {
  transform: translateX(-100%);
  opacity: 0;
}
.fade-enter-to {
  transform: translateX(0%);
  opacity: 1;
}

.fade-leave-from {
  transform: translateX(0%);
  opacity: 1;
}
.fade-leave-to {
  transform: translateX(100%);
  opacity: 0;
}
</style>
