<template>
  <div>
    <div v-if="loading">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
    <div v-else>
      <div id="content" style="display: flex; flex-direction: column; flex: 1;">
        <Ping />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed } from 'vue';
import { AppAgentClient, AppAgentWebsocket } from '@holochain/client';
import '@material/mwc-circular-progress';
import '@material/mwc-button';
import Ping from './ping/ping/Ping.vue';

export default defineComponent({
  components: {
    // Add your subcomponents here
    Ping,
  },
  data(): {
    client: AppAgentClient | undefined;
    loading: boolean;
  } {
    return {
      client: undefined,
      loading: true,
    };
  },
  async mounted() {
    // We pass an unused string as the url because it will dynamically be replaced in launcher environments
    this.client = await AppAgentWebsocket.connect(new URL('https://UNUSED'), 'ping');
    this.loading = false;
  },
  provide() {
    return {
      client: computed(() => this.client),
    };
  },
});
</script>
