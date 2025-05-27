import { createRouter, createWebHistory } from 'vue-router'
import ShortenView from '../views/ShortenView.vue'

const DEFAULT_TITLE = 'unplink'
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: ShortenView,
    },
    {
      path: '/about',
      name: 'about',
      meta: {
        title: `about @ ${DEFAULT_TITLE}`,
      },
      component: () => import('../views/AboutView.vue'),
    },
  ],
})

router.beforeEach((to, _from, next) => {
  document.title = (to.meta.title as string) ?? DEFAULT_TITLE
  next()
})

export default router
