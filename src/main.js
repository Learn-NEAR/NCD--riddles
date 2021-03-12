import Vue from 'vue'
import App from './App.vue'
import Buefy from 'buefy'
import Loading from 'vue-loading-overlay'
import 'buefy/dist/buefy.css'
import 'vue-loading-overlay/dist/vue-loading.css'
import './global.css'

Vue.use(Buefy)
Vue.use(Loading)

import { initContract } from './utils'

Vue.config.productionTip = false

window.nearInitPromise = initContract().then(() => {
  new Vue({
    render: (h) => h(App),
  }).$mount('#app')
})
