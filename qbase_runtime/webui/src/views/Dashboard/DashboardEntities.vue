<script setup lang="ts">
import { Card, Button, DataTable, Listbox, InputText } from 'primevue';
import Enumerable from 'linq';
import { ref } from 'vue';

interface IEntityListOption {
    name: string,
}
interface IEntityField {
    name: string,
    type: EntityFieldType,
    notNull?: boolean,
    base?: boolean,
}
interface IEntity {
    name: string,
    type: EntityType,
    fields: IEntityField[],
    dtos: any
}
enum EntityType {
    AUTH,
    DATA
}
enum EntityFieldType {
    TEXT,
    NUMBER,
    BOOLEAN
}
const entities = ref<IEntityListOption[]>(Enumerable.range(0, 100).select<IEntityListOption>((i) => { return { name: "User" + i } }).toArray())
const selectedEntityScheme: IEntity = {
    fields: [],
    name: 'Users',
    type: EntityType.AUTH,
    dtos: undefined
}
const entityFilter = ref("");
function selectEntity(entity: IEntityListOption) {

}
</script>
<template>
    <div class="dashboard-entities">
        <Listbox class="dashboard-entities-list" :options="entities.filter((e) => e.name.includes(entityFilter))">
            <template #header>
                <InputText v-model="entityFilter"></InputText>
            </template>
            <template #option="{ option }">
                {{ option.name }}
            </template>
        </Listbox>
        <DataTable>

        </DataTable>
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