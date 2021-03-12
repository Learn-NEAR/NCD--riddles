<template>
  <div>
    <button class="link" style="float: right" v-on:click="logout">Sign out</button>
    <main>
      <h1>
        Signed as: {{ accountId }}
      </h1>
    </main>
  </div>
</template>

<script>
import { logout } from "../utils"

export default {
  name: "SignedIn",
  computed: {
    isSignedIn() {
      return window.walletConnection? window.walletConnection.isSignedIn(): false
    },
    accountId() {
      return window.accountId
    },
    contractId() {
      return window.contract? window.contract.contractId: null
    },
    networkId() {
      return window.networkId
    },
  },

  methods: {
    logout,
  },
}
</script>
