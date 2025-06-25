
<script setup lang="ts">
import type { Activity } from './types.ts'

defineProps<{
  title: string,
  processTitle: string,
  steps: string[],
  stepActive: number,
  progressPercentage: number,
  activities: Activity[],
}>()

const progressColors = [
  { color: '#f56c6c', percentage: 20 },
  { color: '#e6a23c', percentage: 40 },
  { color: '#5cb87a', percentage: 60 },
  { color: '#1989fa', percentage: 80 },
  { color: '#6f7ad3', percentage: 100 },
]

</script>
<template>
  <el-space direction="vertical" style="width: 100%; align-items: normal">
    <el-card shadow="always">
      <template #header>
        <span style="font-size: 20px;"><strong>{{ title }}</strong></span>
      </template>

      <div class="card-content">
        <!-- tip -->
        <div class="tip">
          <slot class="tip" name="tip"></slot>
        </div>
        <!-- 参数输入 -->
        <div class="param">
          <slot name="param"></slot>
        </div>

      </div>
    </el-card>

    <!-- 指示区 -->
    <el-card shadow="always">
      <div class="card-content">
        <!-- 步骤提示 -->
        <div class="steps">
          <el-steps direction="vertical" :active="stepActive" finish-status="success">
            <el-step v-for="(step, index) in steps" :key="index" :title="step" />
          </el-steps>
        </div>
        <!-- 进度条 --> 
        <div class="progress">
          <div>{{ processTitle }}</div>
          <div>
            <el-progress :width="200" type="circle" :percentage="progressPercentage" :color="progressColors" />
          </div>
        </div>
        <!-- 时间轴和按钮 -->
        <div class="time-line">
          <div style="display: flex; justify-content: flex-end;">
            <slot name="processButton"></slot>
          </div>
          <el-timeline>
            <el-timeline-item v-for="(activity, index) in activities" :key="index" :timestamp="activity.timestamp">
              {{ activity.content }}
            </el-timeline-item>
          </el-timeline>
        </div>
      </div>
    </el-card>
  </el-space>
</template>

<style scoped>
.tip {
  padding: 8px 16px;
  border-radius: 4px;
  border-left: 5px solid var(--el-color-primary);
  background-color: #f0f5fe;
  flex: 1;
}

.card-content {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 15px;
}

.progress {
  display: flex;
  flex-direction: column;
  gap: 15px;
  flex: 1;
}

.progress div {
  align-self: center;
}

.time-line {
  display: flex;
  flex-direction: column;
  gap: 15px;
}
</style>