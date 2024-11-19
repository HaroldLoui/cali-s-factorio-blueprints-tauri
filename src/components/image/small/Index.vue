<template>
  <el-tabs v-model="activeTab" type="border-card" class="tab-content">
    <el-tab-pane name="tab1" label="显示屏生成">
      <Screen />
    </el-tab-pane>
    <el-tab-pane name="tab2" label="静态图片生成">
      <Static />
    </el-tab-pane>
    <el-tab-pane name="tab3" label="动态图片生成">
      <Dynamic />
    </el-tab-pane>
  </el-tabs>
</template>

<script setup lang="ts">
import { ref } from "vue";

import Screen from "./Screen.vue";
import Static from "./Static.vue";
import Dynamic from "./Dynamic.vue";

const activeTab = ref<string>("tab1");
</script>

<style>
.tab-content>.el-tabs__content {
  height: 550px !important;
}
</style>