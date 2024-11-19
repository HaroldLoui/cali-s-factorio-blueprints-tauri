<template>
  <el-row :gutter="10">
    <el-col :span="8">
      <div class="box_common">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>显示屏大小</span>
            </div>
          </template>
          <el-form :model="form" label-width="auto" style="width: 100%;">
            <el-form-item label="宽度：">
              <el-input-number 
                v-model="form.width" 
                controls-position="right"
                size="small" 
                style="width: 100%;"
                :min="1" 
              />
            </el-form-item>
            <el-form-item label="高度：">
              <el-input-number 
                v-model="form.height" 
                controls-position="right" 
                size="small" 
                style="width: 100%;"
                :min="1" 
              />
            </el-form-item>
            <el-form-item label="">
              <span>注：像素上限为2985</span>
            </el-form-item>
            <el-form-item label="">
              <span>注：必须启用太空时代DLC</span>
            </el-form-item>
          </el-form>
        </el-card>

        <el-card style="margin-top: 5px;">
          <template #header>
            <div class="card-header">
              <span>信号线</span>
            </div>
          </template>
          <el-form :model="form" label-width="auto" style="width: 100%;">
            <el-form-item label="">
              <el-checkbox v-model="form.redLine" label="红线" />
            </el-form-item>
            <el-form-item label="">
              <el-checkbox v-model="form.greenLine" label="绿线" />
            </el-form-item>
          </el-form>
        </el-card>
        <el-card style="margin-top: 5px;">
          <template #header>
            <div class="card-header">
              <span>其它设置</span>
            </div>
          </template>
          <el-form :model="form" label-width="auto" style="width: 100%;">
            <el-form-item label="">
              <el-checkbox v-model="form.keepOpen" label="保持打开" />
            </el-form-item>
          </el-form>
        </el-card>
        <div class="btn">
          <el-button type="primary" plain style="width: 100%;" @click="generateContent">生成蓝图</el-button>
        </div>
      </div>
    </el-col>
    <el-col :span="16">
      <bp-content :bp-content="bpContent" />
    </el-col>
  </el-row>
</template>

<script setup lang="ts">

import { reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import BpContent from "../../common/BpContent.vue";

const form = reactive({
  width: 32,
  height: 32,
  redLine: true,
  greenLine: false,
  keepOpen: true,
});

// 蓝图内容
const bpContent = ref<string>("");

async function generateContent() {
  bpContent.value = await invoke("generate_screen_bp", { form, });
}
</script>