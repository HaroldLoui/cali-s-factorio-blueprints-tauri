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
          <el-form :model="form" style="width: 100%;">
            <el-form-item label-width="auto" label="宽度：">
              <el-input-number 
                v-model="form.width" 
                controls-position="right"
                size="small" 
                style="width: 100%;"
                :min="1" 
              />
            </el-form-item>
            <el-form-item label-width="auto" label="高度：">
              <el-input-number 
                v-model="form.height" 
                controls-position="right" 
                size="small" 
                style="width: 100%;"
                :min="1" 
              />
            </el-form-item>
            <el-form-item label-width="auto" label="帧间间隔(Tick)：">
              <el-input-number 
                v-model="form.tick" 
                controls-position="right" 
                size="small" 
                style="width: 100%;"
                :min="1" 
              />
            </el-form-item>
            <el-form-item label="">
              <span>注：务必和显示屏相同</span>
            </el-form-item>
            <el-form-item label="">
              <span>注：必须启用太空时代DLC</span>
            </el-form-item>
          </el-form>
        </el-card>
        <el-card style="margin-top: 5px;">
          <template #header>
            <div class="card-header">
              <span>图片文件</span>
            </div>
          </template>
          <el-form :model="form" label-width="auto" style="width: 100%;">
            <el-form-item label="">
              <el-image :src="form.showPath">
                <template #error>
                  <div class="image-slot">
                    <span>暂未选择图片</span>
                  </div>
                </template>
              </el-image>
            </el-form-item>
            <div class="image-btns">
              <div class="image-btn">
                <el-button type="success" plain style="width: 100%;" @click="onChooseImage">
                  选择图片
                </el-button>
              </div>
              <div class="image-btn">
                <el-button type="danger" plain style="width: 100%;" @click="form.showPath = ''">
                  清除图片
                </el-button>
              </div>
            </div>
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
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import BpContent from "../../common/BpContent.vue";

const form = reactive({
  width: 32,
  height: 32,
  tick: 1,
  originalPath: "",
  showPath: "",
});

const onChooseImage = async () => {
  const file = await open({
    multiple: false,
    directory: false,
    filters: [{
      name: "",
      extensions: ["gif"]
    }],
  });
  if (file) {
    form.tick = await invoke("get_gif_tick", { path: file });
    form.originalPath = file;
    form.showPath = convertFileSrc(file);
  }
}

// 蓝图内容
const bpContent = ref<string>("");

async function generateContent() {
  if (!form.showPath) {
    ElMessageBox.alert("请选择图片", "错误", {
      type: "error",
      confirmButtonText: "确定",
    })
    return;
  }
  bpContent.value = "生成中...";
  bpContent.value = await invoke("generate_mini_dynamic_image_bp", { form, });
}
</script>

<style scoped>
.el-image {
  width: 202px;
  height: 202px;
}

.image-slot {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
  background: var(--el-fill-color-light);
  color: var(--el-text-color-secondary);
  font-size: 16px;
}

.image-btns {
  width: 100%;
  margin-top: 3px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.image-btn {
  width: 49%;
}
</style>