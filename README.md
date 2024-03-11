# Ping

This repro is useful for demonstrating that holo-dev-server is unable to successfully send or receive remote zome calls, and instead errors:

> Network error: Other: timeout discovering peer

This error is emitted (i.e. the "ping" times out) consistently under the following circumstances:

- The happ is served by a holo-dev-server, and attempts to ping an agent running on another holo-dev-server instance (tested across two linux machines)
- The happ is served by a holo-dev-server and attempts to ping an agent running the webhapp in the holochain launcher
- The webhapp is run by the holochain launcher and attempts to an agent who is served by a holo-dev-server instance

Note that the ping succeeds (though sometimes intermittently) when sent between two agent that are running the webhapp in the holochain launcher, i.e. if neither agent is behind a holo-dev-server.

These issues were reproduced using:

- prebuilt [holochain 0.2.6](https://github.com/holochain-open-dev/holochain-prebuilt-binaries/releases/tag/0.2.6)
- prebuilt [hc 0.2.6](https://github.com/holochain-open-dev/holochain-prebuilt-binaries/releases/tag/0.2.6)
- prebuilt [lair-keystore 0.4.2](https://github.com/holochain-open-dev/holochain-prebuilt-binaries/releases/tag/0.2.6)
- prebuilt [Holochain Launcher v0.11.5](https://github.com/holochain/launcher/releases/tag/v0.11.5)
- holo-dev-server built from src without nix @ [2829a1a](https://github.com/Holo-Host/envoy-chaperone/tree/2829a1afcb6acf5590ee7e1b157ff7e5ca7596fb)

## Using this happ

If you intend to test against both the launcher and holo-dev-server, delete `VITE_IS_HOLO_HOSTED=true` from `ui/.env` and run `npm run package`.

Once you have produced your bundles, you can revert the `.env` change, `cd` into `ui/` and run `npx vite`. This will run up the UI in a mode that expects a locally running instance of holo-dev-server available on port 24274.

Be sure to use consistent bundle builds across the launchers and holo-dev-server instances being tested.
