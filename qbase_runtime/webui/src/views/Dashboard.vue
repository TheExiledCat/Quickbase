<script setup lang="ts">
import { Card, Button, Popover, Listbox } from "primevue";
import { useToast } from "primevue";
import { RouterView, useRouter } from "vue-router";
import Logo from "@/components/Logo.vue";
import { ref } from "vue";
import api from "@/utils/admin_utils";
import { useMessages } from "@/utils/useMessages";
const messages = useMessages();
interface ITab {
  icon: String;
  title: String;
  route: String;
}
const tabs: ITab[] = [
  { icon: "database", title: "entities", route: "/dashboard/entities" },
  { icon: "chart-bar", title: "metrics", route: "/dashboard/metrics" },
  { icon: "cog", title: "settings", route: "/dashboard/settings" },
];
const router = useRouter();
const menuPopover = ref();
const toggleMenu = (event: Event) => {
  menuPopover.value.toggle(event);
};
interface IMenuOption {
  icon: string;
  title: string;
  action: () => void;
}
const menuOptions: IMenuOption[] = [
  {
    icon: "sign-out",
    title: "Sign Out",
    action: () => {
      api.logout();
      router.push("/");
    },
  },
];
messages.addMessage("Quickbase")
</script>
<template>
  <div class="dashboard">
    <div class="dashboard-top-bar">
      <Card v-if="messages.messages.value.length > 0">

        <template #content>
          <div class="dashboard-top-bar-messages">
            <div class="dashboard-top-bar-close">
              <i class="pi pi-times pointer" @click="messages.clearMessages()"></i>
            </div>
            <h1>{{ messages.messages.value[messages.messages.value.length - 1].text }}</h1>

          </div>

        </template>
      </Card>
    </div>

    <div class="dashboard-container">
      <Card>
        <template #content>
          <div class="dashboard-menu dashboard-card">
            <Button class="dashboard-menu-logo-button" @click="toggleMenu">
              <Logo class="dashboard-menu-logo"></Logo>
            </Button>
            <Popover ref="menuPopover">
              <Listbox class="dashboard-menu-logo-popover" :options="menuOptions"
                @change="(event) => event.value.action()">
                <template #option="{ option }">
                  <div class="dashboard-menu-logo-popover-item">
                    <i :class="{ ['pi pi-' + option.icon]: true }"></i>
                    <label>
                      {{ option.title }}
                    </label>
                  </div>
                </template>
              </Listbox>
            </Popover>
            <Button v-for="tab in tabs" class="dashboard-menu-tab-icon" :icon="'pi pi-' + tab.icon"
              @click="router.push(tab.route as string)">
            </Button>
          </div>
        </template>
      </Card>
      <Card class="dashboard-content">
        <template #content>
          <div class="dashboard-card dashboard-main">
            <RouterView></RouterView>
          </div>
        </template>
      </Card>
    </div>
  </div>
</template>
<style>
:root {
  --row-gap: 1rem;

  --column-gap: var(--row-gap);
}

.dashboard {
  height: 100vh;
  width: 100%;
  display: flex;
  flex-direction: column;
  padding-inline: 2rem;
  padding-block-start: 1%;
  padding-block-end: 1%;
  gap: var(--column-gap);
  overflow: hidden;
}

.dashboard-top-bar {
  max-height: 15%;
  overflow: auto;
  height: fit-content;
}

.dashboard-top-bar * {
  width: 100%;
  height: 100%;
}

.dashboard-top-bar-messages {
  width: 100%;
  position: relative;
}

.dashboard-top-bar-close {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: fit-content;
  align-items: flex-end;
  text-align: end;
  position: absolute;
  right: 0;
  top: 0
}

.dashboard-container {
  display: flex;
  flex-direction: row;
  gap: calc(var(--row-gap) / 2);

  width: 100%;
  height: 100%;
  --menu-width: 4rem;
  overflow: hidden;
}



.dashboard-menu {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: fit-content;
  max-width: var(--menu-width);
  gap: var(--row-gap);
}

.dashboard-menu-logo-button {
  padding: 0 !important;
}

.dashboard-menu-logo-popover-item {
  display: flex;
  gap: 1rem;
  text-wrap: nowrap;
  justify-content: space-around;
  align-items: center;
  text-align: center;
}

.dashboard-menu-logo-popover-item * {
  pointer-events: none;
}

.dashboard-menu-logo {
  width: 100%;
  height: 100%;
}

.dashboard-card {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.dashboard-content {
  height: 100%;
}

.dashboard-container,
.dashboard-main {
  width: 100%;
}

.dashboard-content>.p-card-body,
.dashboard-content>.p-card-body>.p-card-content {
  height: 100%;
}

.dashboard-content,
.dashboard-main {
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.dashboard-menu-tab-icon {
  aspect-ratio: 1/1;
  width: 75%;
}

.dashboard-menu-tab-icon * {
  font-size: calc(var(--menu-width) - 3rem);
}
</style>
