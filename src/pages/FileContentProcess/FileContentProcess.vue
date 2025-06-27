<script setup lang="ts">
import { ref, watch } from 'vue'
import FileProcessLayout from '../../components/FileProcessLayout.vue'
import { Activity } from '../../components/types'
import { FileContentProcessParam } from './types'
import ParamForm from './ParamForm.vue'

const steps = ref<string[]>(['选择要搜索的文件', '输入要搜索的字符串', '开始搜索'])
const progressPercentage = ref(0)
const activities = ref<Activity[]>([])
const stepActive = ref(0)
const changeStepActive = (step: number) => {
  stepActive.value = step
}
const fileContentProcessForm = ref<FileContentProcessParam>({
  filePaths: '',
  outputDir: '',
  searchString: '',
  regex: false,
  isReplace: false,
  replaceString: '',
})

watch(() => fileContentProcessForm.value.isReplace, (newVal) => {
  steps.value = newVal
    ? ['选择要替换的文件', '输入要替换的字符串', '开始替换']
    : ['选择要搜索的文件', '输入要搜索的字符串', '开始搜索']
})

const searchFile = () => {
  console.log(fileContentProcessForm.value)
}
</script>
<template>
  <FileProcessLayout title="文件内容处理" processTitle="处理进度" :steps="steps" :stepActive="stepActive"
    :progressPercentage="progressPercentage" :activities="activities">
    <template #tip>
      <p>此功能用于在文件内容中搜索指定字符串{{ fileContentProcessForm.isReplace ? '并进行替换' : '' }}</p>
      <p>操作步骤：</p>
      <p>1. 选择要{{ fileContentProcessForm.isReplace ? '替换' : '搜索' }}的文件(可多选)</p>
      <p>2. 输入要{{ fileContentProcessForm.isReplace ? '替换' : '搜索' }}的字符串</p>
      <p>3. 点击【开始{{ fileContentProcessForm.isReplace ? '替换' : '搜索' }}】按钮开始处理</p>
    </template>

    <template #param>
      <ParamForm v-model:fileContentProcessForm="fileContentProcessForm" @changeStepActive="changeStepActive" />
    </template>

    <template #processButton>
      <el-button v-show="fileContentProcessForm.filePaths !== '' && fileContentProcessForm.searchString !== ''" 
        type="primary" @click="searchFile">{{ fileContentProcessForm.isReplace ? '开始替换' : '开始搜索' }}</el-button>
    </template>
  </FileProcessLayout>
	<div>
    <!-- todo 展示搜索到的内容 分文件名tab -->
  </div>
</template>
