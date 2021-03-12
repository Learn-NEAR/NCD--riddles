import Vue from 'vue'
import App from './App.vue'
import Buefy from 'buefy'
import 'buefy/dist/buefy.css'
import './global.css'

Vue.use(Buefy)

import { initContract } from './utils'

Vue.config.productionTip = false

window.nearInitPromise = initContract().then(() => {
  new Vue({
    render: (h) => h(App),
  }).$mount('#app')
})
