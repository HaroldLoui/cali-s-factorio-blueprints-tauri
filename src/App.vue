<template>
  <el-tabs v-model="activeTab" type="border-card">
    <el-tab-pane name="tab1" label="欢迎页面">
      <Index />
    </el-tab-pane>
    <el-tab-pane name="tab2" label="显示屏生成">
      <Screen />
    </el-tab-pane>
    <el-tab-pane name="tab3" label="图片生成">
      <Image />
    </el-tab-pane>
  </el-tabs>
</template>

<script setup lang="ts">
import { ref } from "vue";

import Index from "./components/Index.vue";
import Screen from "./components/Screen.vue";
import Image from "./components/Image.vue";

const activeTab = ref<string>("tab1");

</script>

<style>
.el-tabs__content {
  height: 510px;
}

.el-col {
  height: 510px;
}

.el-card__header,
.el-card__body {
  padding-top: 5px !important;
  padding-bottom: 5px !important;
}

.el-form-item {
  margin-bottom: 0 !important;
}

.box_common {
  height: 100%;
  position: relative;
}

.btn {
  position: absolute;
  bottom: 5px;
  width: 100%;
}
</style>