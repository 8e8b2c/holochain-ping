<template>
    <mwc-snackbar ref="ping-error"></mwc-snackbar>

    <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">This agent: {{ myPubKey }}</span>

        <span style="font-size: 18px">Ping Agent</span>

        <div style="margin-bottom: 16px">
            <mwc-textfield outlined label="Recipient" :value="recipient" @input="recipient = $event.target.value"
                required></mwc-textfield>
        </div>


        <mwc-button raised label="Ping" :disabled="!isRecipientValid" @click="pingRecipient"></mwc-button>

        <span style="font-size: 18px">Response: {{ response }}</span>

    </div>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash, encodeHashToBase64, decodeHashFromBase64 } from '@holochain/client';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textfield';
export default defineComponent({
    data(): {
        recipient: string;
        response: string;
    } {
        return {
            recipient: '',
            response: '',
        }
    },
    computed: {
        isRecipientValid() {
            return true && this.recipient !== '';
        },
        myPubKey() {
            return encodeHashToBase64(this.client.myPubKey)
        }
    },
    mounted() {
    },
    methods: {
        async pingRecipient() {
            const agent = decodeHashFromBase64(this.recipient);
            this.response = '';

            try {
                const response = await this.client.callZome({
                    cap_secret: null,
                    role_name: 'ping',
                    zome_name: 'ping',
                    fn_name: 'ping_agent',
                    payload: agent,
                });
                this.response = `${response}  (${new Date().toISOString()})`
            } catch (e: any) {
                const errorSnackbar = this.$refs['ping-error'] as Snackbar;
                errorSnackbar.labelText = `Error executing ping_agent: ${e.data}`;
                errorSnackbar.show();
            }
        },
    },
    setup() {
        const client = (inject('client') as ComputedRef<AppAgentClient>).value;
        return {
            client,
        };
    },
})
</script>