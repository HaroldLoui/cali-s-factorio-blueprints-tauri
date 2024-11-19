<template>
  <el-tabs v-model="activeTab" type="border-card" class="app-tab-content">
    <el-tab-pane name="tab1" label="欢迎页面">
      <Welcome />
    </el-tab-pane>
    <el-tab-pane name="tab2" label="小彩图生成">
      <SmallImage />
    </el-tab-pane>
    <el-tab-pane name="tab3" label="大彩图生成（施工中）">
      大彩图生成
    </el-tab-pane>
    <el-tab-pane name="tab4" label="蓝图编辑（施工中）">
      大彩图生成
    </el-tab-pane>
  </el-tabs>
</template>

<script setup lang="ts">
import { ref } from "vue";

import Welcome from "./components/Welcome.vue";
import SmallImage from "./components/image/small/Index.vue";

const activeTab = ref<string>("tab1");
</script>

<style>
.app-tab-content>.el-tabs__content {
  height: 620px;
}

.el-col {
  height: 550px;
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