<script setup lang="ts">
import { useDark } from '@vueuse/core';
const isDark = useDark();
</script>
<template>
    <div class="logo">
        <img class="logo-img" :src="isDark? '/Qbaselogo_WhiteOnBlack.svg': '/Qbaselogo_BlackOnWhite.svg'">
    </div>
</template>
<style scoped>
.logo-img{
    width: 100%;
    height: 100%;
    object-fit: cover;
    image-resolution: from-image 1000dpi;
}
</style>