import { createRouter, createWebHistory } from 'vue-router'
import IndexPage from '../pages/index.vue'

const DEFAULT_TITLE = 'unplink'
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: IndexPage,
    },
    {
      path: '/about',
      name: 'about',
      meta: {
        title: `about @ ${DEFAULT_TITLE}`,
      },
      component: () => import('../pages/about.vue'),
    },
  ],
})

router.beforeEach((to, _from, next) => {
  document.title = (to.meta.title as string) ?? DEFAULT_TITLE
  next()
})

export default router
