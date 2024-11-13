CCTray Proxy
---


### How to run it?

1. Prepare your configuration file

create token which only have **pipeline readonly** permission.
See ![img](./docs/images/generate-token.png)

save the token and repo id in a configuration file locally.

Refer to [example config](./config/example-config.json)

2. Run the following command
```bash
/Applications/cctray-proxy <path-to-repo-config>
```