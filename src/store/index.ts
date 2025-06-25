import { createStore } from 'vuex'
import { actions, mutations, type State } from 'papudding-layout'
import { menuItemsBuilder } from '../utils/menuItemBuilder.ts'
import router, { pagesRoutes } from '../router'

const menuItems = menuItemsBuilder(router)

export const store = createStore<State>({
  state () {
    return {
      tabList: [{
        path: '/home',
        title: '功能概览',
        tabPath: ['功能概览']
      }],
      activeTab: '/home',
      breadcrumbItemList: ['功能概览'],
      menuItems: menuItems,
      pagesRoutes: pagesRoutes,
      avatarUrl: 'https://avatars.githubusercontent.com/u/10262924?v=4',
    }
  },
  mutations: {
    ...mutations
  },
  actions: {
    ...actions
  }
})
