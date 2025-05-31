<script setup lang="ts">
import { Card, Button } from "primevue";
import { RouterView } from "vue-router";
interface ITab {
  icon: String;
  title: String;
  route: String;
}
const tabs: ITab[] = [
  { icon: "database", title: "entities", route: "/dasboard/entities" },
  { icon: "chart-bar", title: "metrics", route: "/dasboard/metrics" },
  { icon: "cog", title: "settings", route: "/dasboard/settings" },
];
</script>
<template>
  <div class="dashboard">
    <Card>
      <template #content>
        <div class="dashboard-menu dashboard-card">
          <Button disabled>
            <img src="/favicon.ico" />
          </Button>
          <Button
            v-for="tab in tabs"
            class="dashboard-menu-tab-icon"
            :icon="'pi pi-' + tab.icon"
          >
          </Button>
        </div>
      </template>
    </Card>
    <Card class="dashboard-container">
      <template #content>
        <div class="dashboard-card dashboard-main">
          <RouterView></RouterView>
        </div>
      </template>
    </Card>
  </div>
</template>
<style scoped>
.dashboard {
  display: flex;
  flex-direction: row;
  gap: calc(var(--row-gap) / 2);
  padding-inline: 0.8rem;
  padding-block-start: 5%;
  padding-block-end: 1%;
  width: 100%;
  height: 100%;
  --row-gap: 1rem;
}
.dashboard-menu {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: fit-content;
  gap: var(--row-gap);
}
.dashboard-card {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
}
.dashboard-container,
.dashboard-main {
  width: 100%;
}
.dashboard-menu-tab-icon {
  aspect-ratio: 1/1;
  font-size: 3rem;
}
</style>
