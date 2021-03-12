<template>
  <div>
    <h3>Answer Riddles</h3>
    <br />

    <b-field label="Pick one riddle"></b-field>
    <b-table
      :data="questionList"
      :bordered="isBordered"
      :striped="isStriped"
      :narrowed="isNarrowed"
      :hoverable="isHoverable"
      :loading="isLoading"
      :focusable="isFocusable"
      :paginated="isPaginated"
      :per-page="perPage"
      :row-class="(row, index) => selectedQuestionId == row.id && 'is-selected'"
      @select="onSelect"
    >
      <b-table-column label="#" width="10" centered v-slot="props">
        {{ props.row.id }}
      </b-table-column>

      <b-table-column label="Creator" width="20" v-slot="props">
        {{ props.row.creator }}
      </b-table-column>

      <b-table-column label="Question" width="20" v-slot="props">
        {{ props.row.riddle_info.question }}
      </b-table-column>

      <b-table-column label="Kind" width="20" v-slot="props">
        {{ props.row.riddle_info.kind }}
      </b-table-column>

      <b-table-column label="Bonus" width="20" v-slot="props">
        {{ convertBonus(props.row.bonus) }}
      </b-table-column>

      <b-table-column label="Difficulty" width="10" v-slot="props">
        {{ props.row.grade }}
      </b-table-column>

      <div slot="empty" class="has-text-centered">
        No Riddle
      </div>
    </b-table>

    <b-field v-if="selectedQuestionId" label="You have picked:" class="has-text-centered">
      <div>{{ selectedQuestionId }}</div>
    </b-field>

    <b-field label="Answer">
      <b-input v-model="answer" placeholder="enter the answer"></b-input>
    </b-field>

    <br />

    <b-field>
      <b-button @click="answer_riddle">Submit</b-button>
    </b-field>
  </div>
</template>

<script>
import sha256 from 'js-sha256'

export default {
  name: 'AnswerRiddle',
  data() {
    return {
      isEmpty: false,
      isBordered: false,
      isStriped: false,
      isNarrowed: false,
      isHoverable: false,
      isFocusable: false,
      isPaginated: false,
      perPage: 8,
      isLoading: false,
      selectedQuestionId: null,
      questionList: [],
      answer: null,
      loader: null,
    }
  },
  methods: {
    answer_riddle: async function() {
      if (!this.selectedQuestionId) {
        window.alert("You haven't selected any question")
        return
      }

      if (!this.answer) {
        window.alert("You haven't filled the form'")
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

        await window.contract.answer_riddle({
          answer: {
            id: this.selectedQuestionId,
            sha256_answer: sha256(this.answer),
          },
        })

        this.questionList = await window.contract.get_riddles()
        this.answer = null
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
    onSelect(row) {
      if (row && row.id) {
        this.selectedQuestionId = row.id
      }
    },
    convertBonus(bonus) {
      return +bonus / Math.pow(10, 24)
    },
  },

  mounted: async function() {
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

      this.questionList = await window.contract.get_riddles()
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
}
</script>

<style type="text/css">
.b-table table.tr.active {
  color: #fff;
  background-color: blue;
}
</style>
