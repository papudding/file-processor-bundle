<script setup lang="ts">
import { ref } from 'vue'
import { FileMergeParam } from './types'
import { open } from '@tauri-apps/plugin-dialog'
import { ElNotification } from 'element-plus'

const emit = defineEmits<{
  (e: 'changeStepActive', step: number): void
}>()

const fileMergeForm = defineModel<FileMergeParam>('fileMergeForm', {
  default: () => ({
    filePaths: '',
    outputDir: '',
    outputFileName: '',
    parallel: false,
  })
})
const fileNames = ref<string>('')
const fileMergeFormRules = ref({
  filePaths: [
    { required: true, message: '请选择文件', trigger: 'blur' },
  ],
  outputDir: [
    { required: true, message: '请选择输出目录', trigger: 'blur' },
  ],
})

// 处理选择文件
const selectFile = async () => {
  // Open a dialog
  const files = await open({
    multiple: true,
    directory: false,
  })

  if (files && files.length) {
    if (files.length === 1) {
      ElNotification({
        title: '错误',
        message: '请选择多个文件',
        type: 'error',
      })
      return
    }
    fileNames.value = files.map((file) => file.split(/[\\/]/).pop() || '').join('\n')
    fileMergeForm.value.filePaths = files.join(',')
    emit('changeStepActive', 2)
  }
}
// 选择目录
const selectDir = async () => {
  const file = await open({
    multiple: false,
    directory: true,
  })
  // 处理 file 可能为 null 的情况，若为 null 则赋值为空字符串
  fileMergeForm.value.outputDir = file || ''
}
</script>
<template>
  <el-form style="min-width: 330px;" label-position="top" :model="fileMergeForm" :rules="fileMergeFormRules">
    <el-form-item label="待合并文件" prop="filePaths">
      <el-row>
        <el-col :span="19">
          <el-input :autosize="{ minRows: 2, maxRows: 5 }" type="textarea" readonly v-model="fileNames"
            placeholder="点击按钮选择文件" />
          <el-input v-show="false" hidden readonly v-model="fileMergeForm.filePaths" />
        </el-col>
        <el-col :span="4" :offset="1">
          <el-button type="primary" @click="selectFile">选择文件</el-button>
        </el-col>
      </el-row>
    </el-form-item>
    <el-form-item label="合并后的文件路径" prop="outputDir">
      <el-row>
        <el-col :span="19">
          <el-input readonly v-model="fileMergeForm.outputDir" placeholder="点击按钮选择输出路径" />
        </el-col>
        <el-col :span="4" :offset="1">
          <el-button type="primary" @click="selectDir">选择输出</el-button>
        </el-col>
      </el-row>
    </el-form-item>
    <el-row>
      <el-col :span="12">
        <el-form-item prop="outputFileName">
          <template #label>
            <span>合并后的文件名
              <el-icon>
                <el-tooltip placement="right" content="未指定则默认为“bundle”">
                  <InfoFilled />
                </el-tooltip>
              </el-icon>
            </span>
          </template>
          <el-input v-model="fileMergeForm.outputFileName" placeholder="请输入合并后的文件名" />
        </el-form-item>
      </el-col>
      <el-col :span="11" :offset="1">
        <el-form-item prop="parallel">
          <template #label>
            <span>是否并行
              <el-icon>
                <el-tooltip placement="bottom" content="并行意味着乱序，慎重选择">
                  <InfoFilled />
                </el-tooltip>
              </el-icon>
            </span>
          </template>
          <el-switch v-model="fileMergeForm.parallel" :active-value="true" :inactive-value="false" :active-text="'是'" :inactive-text="'否'" inline-prompt/>
        </el-form-item>
      </el-col>
    </el-row>
  </el-form>
</template>