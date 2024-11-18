<template>
  <div class="box_common">
    <div class="bp_content_box">{{ bpContent }}</div>
    <div class="btn" style="width: 510px;">
      <el-button type="primary" style="width: 100%;" @click="copyContent">复制蓝图</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
// import { ref } from "vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

const props = defineProps({
  bpContent: {
    type: String,
    required: true,
  }
});
// const bpContent = ref<string>("");

const copyContent = async () => {
  if (props.bpContent) {
    await writeText(props.bpContent);
    ElMessage({
      message: "复制成功",
      type: "success",
      offset: 480,
    });
  }
}
</script>

<style scoped>
.bp_content_box {
  padding: 5px;
  width: 100%;
  height: 460px;
  border: 1px solid #ccc;
  overflow-y: auto;
  word-wrap: break-word;
}
</style>