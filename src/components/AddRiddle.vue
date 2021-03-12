<template>
  <div>
    <h3>Add new Riddle</h3>

    <br />

    <b-field label="question">
      <b-input v-model="question" placeholder="enter the question"></b-input>
    </b-field>
    <b-field label="answer">
      <b-input v-model="answer" placeholder="enter the answer"></b-input>
    </b-field>
    <b-field label="kind">
      <b-select v-model="kind" expanded placeholder="select the kind">
        <option v-for="k of kindList" :value="k" :key="k">{{ k }}</option>
      </b-select>
    </b-field>
    <b-field label="Grade">
      <b-select v-model="grade" expanded placeholder="select the difficulty">
        <option v-for="g of gradeList" :value="g" :key="g">{{ g }}</option>
      </b-select>
    </b-field>
    <b-field label="bonus">
      <b-select v-model="bonus" expanded placeholder="select the bonus">
        <option v-for="b of bonusList" :value="b" :key="b">{{ b }}</option>
      </b-select>
    </b-field>

    <br />

    <b-field>
      <b-button @click="add_riddle">Submit</b-button>
    </b-field>
  </div>
</template>

<script>
import sha256 from 'js-sha256'
import BN from 'bn.js'

export default {
  name: 'AddRiddle',
  data() {
    return {
      question: null,
      answer: null,
      kind: 'History',
      kindList: ['History', 'Culture', 'Science', 'Math', 'Other'],
      grade: 'Easy',
      gradeList: ['Easy', 'Medium', 'Hard'],
      bonus: 1,
      bonusList: [0.5, 1, 2, 4, 8, 16],
      loader: null,
    }
  },
  methods: {
    add_riddle: async function() {
      if (!this.question || !this.answer) {
        window.alert("You haven't filled the form")
        return
      }

      try {
        this.loader = this.$loading.show({
          container: null,
          width: 124,
          height: 124,
          loader: 'bars',
          canCancel: false,
          color: '#CDE201',
          backgroundColor: '#000000',
          opacity: 0.3,
          zIndex: 1002,
        })

        console.log(
          JSON.stringify({
            input: {
              riddle: {
                question: this.question,
                sha256_answer: sha256(this.answer),
                kind: this.kind,
              },
              grade: this.grade,
            },
          })
        )

        const NearUnit = new BN('10000000000000000000000')
        const bonus = NearUnit.muln(+this.bonus * 100)

        await window.contract.add_riddle(
          {
            input: {
              riddle: {
                question: this.question,
                sha256_answer: sha256(this.answer),
                kind: this.kind,
              },
              grade: this.grade,
            },
          },
          new BN('30000000000000'),
          bonus
        )

        window.alert('Successfully created riddle')
      } catch (e) {
        window.alert(
          'Something went wrong! ' +
            'Maybe you need to sign out and back in? ' +
            'Check your browser console for more info.'
        )
      } finally {
        if (this.loader) {
          this.loader.hide()
          this.loader = null
        }
      }
    },
  },
}
</script>

<style type="text/css"></style>
