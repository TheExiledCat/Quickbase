<script setup lang="ts">
import { Card, Button, DataTable, Listbox, InputText } from "primevue";
import Enumerable from "linq";
import { ref } from "vue";
import api from "@/utils/admin_utils";
import type { Schema } from "@/classes/Schema";
import type { Entity } from "@/classes/Entity";
const schema = ref<Schema>();
api.getSchema().then((res) => {
  schema.value = res.data;
});

const selectedEntityScheme = ref<Entity>();
const entityFilter = ref("");
</script>
<template>
  <div class="dashboard-entities">
    <Listbox class="dashboard-entities-list" :options="schema?.entities">
      <template #header>
        <InputText v-model="entityFilter"></InputText>
      </template>
      <template #option="{ option }">
        {{ option.name }}
      </template>
    </Listbox>
    <DataTable> </DataTable>
  </div>
</template>
<style>
.dashboard-entities {
  display: flex;
  flex-direction: row;
  gap: var(--column-gap);
  justify-content: space-between;
  width: 100%;
  height: 100%;
}

.dashboard-entities-list,
.dashboard-entities-list * {
  max-height: fit-content !important;
  width: 100%;
  text-align: center !important;
  display: flex;
  align-items: center;
  flex-direction: column;
}

.dashboard-entities-list {
  width: 20%;
  height: 100%;
}

.dashboard-entities-list .p-listbox-list {
  height: auto;
}

.dashboard-entities-list .p-listbox-option {
  height: fit-content;
}
</style>
