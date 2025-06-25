import { createMemoryHistory, createRouter, RouteRecordRaw } from 'vue-router'
import { InfoFilled, Files, Paperclip, MagicStick, Document } from '@element-plus/icons-vue'
import { PapuddingSkeleton } from 'papudding-layout'
import { markRaw } from 'vue'

export const pagesRoutes: RouteRecordRaw[] = [
  {
    path: '/home',
    component: () => import('./pages/ToolsGuide.vue'),
    meta: {
      icon: markRaw(InfoFilled),
      title: '功能概览'
    }
  },
  {
    path: '/files',
    meta: {
      icon: markRaw(Files),
      title: '文件处理'
    },
    children: [
      {
        path: '/merge',
        component: () => import('./pages/FileMerge/FileMerge.vue'),
        meta: {
          icon: markRaw(MagicStick),
          title: '文件合并'
        }
      },
      {
        path: '/split',
        component: () => import('./pages/FileSplit/FileSplit.vue'),
        meta: {
          icon: markRaw(Paperclip),
          title: '文件拆分'
        }
      }
    ]
  },
  {
    path: '/excel',
    meta: {
      icon: markRaw(Document),
      title: 'Excel处理'
    },
    children: [
      // {
      //   path: '/merge',
      //   component: () => import('./pages/HelloWorld.vue'),
      //   meta: {
      //     icon: markRaw(MagicStick),
      //     title: '文件合并'
      //   }
      // },
      // {
      //   path: '/split',
      //   component: () => import('./pages/FileSplit.vue'),
      //   meta: {
      //     icon: markRaw(Paperclip),
      //     title: '文件拆分'
      //   }
      // }
    ]
  }
]

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    children: pagesRoutes,
    component: PapuddingSkeleton
  },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export default router