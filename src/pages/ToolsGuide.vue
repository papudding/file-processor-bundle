<script setup lang="ts">
import { pagesRoutes } from '../router.ts'
import { onMounted, shallowRef, ref } from 'vue'
import { type RouteRecordRaw, useRouter } from 'vue-router'
import { useStore } from 'vuex'
import { key, buildTab } from 'papudding-layout'

const store = useStore(key)
const router = useRouter()
const descMap = new Map<string, string>(
  [
    ['/merge', '此功能用于将用户选择的【多个文件】合并成一个文件'],
    ['/split', '此功能用于将【单个文件】按用户指定的【最大行数】拆分为多个文件'],
    ['/content-process', '此功能用于在文件内容中【搜索】或【替换】指定字符串，【搜索】和【替换】不会修改原始文件，只会将搜索到的内容输出到新文件中'],
  ]
)

onMounted(() => {
  routes.value = pagesRoutes.filter(item => item.path !== '/home')
})
const routes = shallowRef<RouteRecordRaw[]>([])
const activeNames = ref<string[]>([...pagesRoutes.map(item => item.path)])

// 阻止折叠
const beforeCollapse = ():Promise<boolean> => {
  return new Promise((resolve) => {
    resolve(false)
  })
}

const handleClick = (path:string) => {
  store.dispatch('switchPage', buildTab(path, pagesRoutes))
  router.push(path)
}
</script>
<template>
  <el-collapse class="tools-guide" v-model="activeNames" :before-collapse="beforeCollapse">
    <el-collapse-item v-for="typeItem in routes" :key="typeItem.path" :name="typeItem.path">
      <template #title>
        <component style="margin: 0px 8px 0px 8px; color: grey" class="papudding-layout-menu-icon" :is="typeItem.meta?.icon" />
        <span style="font-size: 16px; color: grey">{{ typeItem.meta?.title }}</span>
      </template>
      <template #icon>&nbsp;</template>

      <div class="tools-guide-cards">
        <el-card style="width: 360px" shadow="hover" v-for="item in typeItem.children" :key="item.path" @click="handleClick(item.path)">
          <template #header>
            <component style="margin: 0px 8px 0px 0px" class="papudding-layout-menu-icon" :is="item.meta?.icon" />
            <span style="font-size: 18px;">{{ item.meta?.title }}</span>
          </template>
          <p style="margin-top: 10px">{{ descMap.get(item.path) }}</p>
        </el-card>
      </div>

    </el-collapse-item>
  </el-collapse>
</template>
<style scoped>
.tools-guide {
  border-top: none;
}
.tools-guide :deep(.el-collapse-item__header) {
  background-color: #f0f2f7;
  border-bottom: none;
  cursor: default;
}
.tools-guide :deep(.el-collapse-item__content) {
  background-color: #f0f2f7;
}
.tools-guide :deep(.el-collapse-item__wrap) {
  border-bottom: none;
}

.tools-guide-cards {
  padding: 5px;
  display: flex;
  flex-wrap: wrap;
  gap: 25px;
  background-color: #f0f2f7;
  cursor: pointer;
}

</style>