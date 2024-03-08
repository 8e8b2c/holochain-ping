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
import WebSdkApi, { ChaperoneState } from '@holo-host/web-sdk';
import '@material/mwc-circular-progress';
import '@material/mwc-button';
import Ping from './ping/ping/Ping.vue';

const IS_HOLO_HOSTED = import.meta.env.VITE_IS_HOLO_HOSTED;

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
    if (IS_HOLO_HOSTED) {

      const holoClient = await WebSdkApi.connect({
        chaperoneUrl: 'http://localhost:24274',
      });
      await new Promise<void>(resolve => {
        const handleHoloChaperoneStateChange = (state: ChaperoneState) => {
          if (state.uiState.isVisible) return;
          if (!state.agentState.isAvailable) return;

          if (state.agentState.isAnonymous) holoClient.signUp({});
          else resolve();
        };
        holoClient.on('chaperone-state', handleHoloChaperoneStateChange);
        if (holoClient.chaperoneState) handleHoloChaperoneStateChange(holoClient.chaperoneState);
      })

      this.client = holoClient as unknown as AppAgentWebsocket;
    } else {
      this.client = await AppAgentWebsocket.connect(new URL('https://UNUSED'), 'ping');
    }

    this.loading = false;
  },
  provide() {
    return {
      client: computed(() => this.client),
    };
  },
});
</script>
