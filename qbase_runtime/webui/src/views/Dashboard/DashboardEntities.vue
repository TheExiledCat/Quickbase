<script setup lang="ts">
import { Card, Button, DataTable, Column, Listbox, InputText } from "primevue";
import Enumerable from "linq";
import { ref } from "vue";
import api from "@/utils/admin_utils";
import type { Schema } from "@/classes/Schema";
import type { Entity } from "@/classes/Entity";
const schema = ref<Schema>();
api.getSchema().then((res) => {
    schema.value = res.data;
    selectEntity(schema.value.entities[0]);
});

const selectedEntityScheme = ref<Entity>();
const entityFilter = ref("");
const rows = Enumerable.range(0, 100).select(i => { return { id: "A" + i, created: "B", updated: "C" } }).toArray();
function selectEntity(entity: Entity) {
    if (entity)
        selectedEntityScheme.value = entity;
}
const selectedRows = ref([])
</script>
<template>
    <div class="dashboard-entities">
        <Listbox class="dashboard-entities-list" :options="schema?.entities" @change="(e) => selectEntity(e.value)">
            <template #header>
                <h3>Entities</h3>

                <InputText v-model="entityFilter"></InputText>
            </template>
            <template #option="{ option }">
                {{ option.name }}
            </template>
        </Listbox>
        <Transition name="fade">
            <Card class="dashboard-entities-table-container" v-if="selectedEntityScheme">
                <template #content>
                    <div class="dashboard-entities-table-card">
                        <InputText placeholder="filter" class="dashboard-entities-table-filter"></InputText>
                        <DataTable v-model:selection="selectedRows" :value="rows" class="dashboard-entities-table"
                            show-gridlines stripedRows sort-field="id" scrollable selectionMode="multiple">

                            <Column selectionMode="multiple">
                            </Column>
                            <Column v-for="field in selectedEntityScheme?.fields" :header="field.name"
                                :field="field.name.toLowerCase()" sortable>
                            </Column>
                        </DataTable>
                    </div>

                </template>
            </Card>
        </Transition>

    </div>
</template>
<style>
.dashboard-entities {
    display: flex;
    flex-direction: row;
    gap: var(--column-gap);
    justify-content: flex-start;
    width: 100%;
    height: 100%;
}

.dashboard-entities-list,
.dashboard-entities-list * {
    width: 100%;
    text-align: center !important;
    display: flex;
    align-items: center;
    flex-direction: column;
}

.dashboard-entities-list * {
    max-height: fit-content !important;
}

.dashboard-entities-list {
    width: 15%;
    height: 100%;
}

.dashboard-entities-list .p-listbox-list {
    height: auto;
}

.dashboard-entities-list .p-listbox-option {
    height: fit-content;
}

.dashboard-entities-table-container {
    display: flex;
    width: 100%;
    height: 100%;
    background: var(--p-surface-700) !important;
    padding: 0 !important;
}

.dashboard-entities-table {
    width: 100%;
    border-radius: var(--p-card-border-radius);
    height: 100%;
    overflow: hidden;
}


.dashboard-entities-table-container .p-card-body,
.dashboard-entities-table .p-datatable-table-container,
.dashboard-entities-table-container .p-card-content {
    height: 100%;
}

.p-datatable-thead,
.p-datatable-thead * {
    height: auto;
}

.dashboard-entities-table-card {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    overflow: hidden;
}


.dashboard-entities-table-filter {
    margin-block-end: .5rem;
}
</style>
