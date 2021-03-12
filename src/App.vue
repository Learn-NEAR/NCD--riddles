<template>
  <div id="root" class="section">
    <header class="level">
      <div class="level-left">
        <b-button @click="toggleMode">
          {{
            !isAddRiddleMode
              ? 'I would like to add new riddle'
              : 'I would like to answer riddle'
          }}
        </b-button>
      </div>
      <div class="level-right">
        <b-button v-show="!isSignedIn" @click="login">Sign In</b-button>
        <b-button v-show="isSignedIn" @click="logout">Sign Out</b-button>
      </div>
    </header>

    <main>
      <div v-if="!isSignedIn">
        <h1>Welcome to NEAR RIDDLE GAME!</h1>
        <p>
          To start playing the game, you need to sign in. The button below will
          sign you in using NEAR Wallet.
        </p>
        <br />
        <p>
          By default, it connects to a test network ("testnet") wallet. This
          works just like the main network ("mainnet") wallet, but the NEAR
          Tokens on testnet aren't convertible to other currencies - so you can
          play without worrying about money!
        </p>
        <br />
      </div>
      <div v-if="isSignedIn">
        <AddRiddle v-if="isAddRiddleMode"></AddRiddle>
        <AnswerRiddle v-if="!isAddRiddleMode"></AnswerRiddle>
      </div>
    </main>

    <footer></footer>
  </div>
</template>

<script>
import getConfig from './config'
import AddRiddle from './components/AddRiddle.vue'
import AnswerRiddle from './components/AnswerRiddle.vue'
import { login, logout } from './utils.js'

const nearConfig = getConfig('development')
console.log(
  `networkId:${nearConfig.networkId} CONTRACT_NAME:${nearConfig.contractName}`
)
window.networkId = nearConfig.networkId

export default {
  created() {
    document.title = 'near-riddles'
  },
  name: 'App',
  components: {
    AddRiddle,
    AnswerRiddle,
  },
  data() {
    return {
      // mode can be either `addRiddle` ot `answerRiddle`
      mode: 'addRiddle',
    }
  },
  methods: {
    toggleMode() {
      if (this.mode == 'addRiddle') {
        this.mode = 'answerRiddle'
      } else {
        this.mode = 'addRiddle'
      }
    },
    logout,
    login,
  },
  computed: {
    isAddRiddleMode() {
      return this.mode == 'addRiddle'
    },
    isSignedIn() {
      return window.walletConnection.isSignedIn()
    },
  },
}
</script>
