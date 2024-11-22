CCTray Proxy
---
CCTray Proxy is a CLI tool that run  on you local machine. Currently only supprts MacOS.

The CLI tool read a configuration file and translate Bitbucket Pipeline's pipeline status
to [cctray](https://www.cctray.org) feed.

The cctray feed can be configured in CCMenu.

### Install

Go to the [release page](https://github.com/nicholasren/cctray-proxy/releases/) and download the latest
`cctray-proxy.pkg.zip`.



### Run the cli App

1. Prepare your configuration file

create token which only have **pipeline readonly** permission.
See ![img](./docs/images/generate-token.png)

save the token and repo id in a configuration file locally.

Refer to [example config](./config/example-config.json)

2. Run the following command
```bash
~/Applications/cctray-proxy <path-to-repo-config>
```
3. Access the proxy via http://localhost:3000/cctray.xml
4. Config CCMenu to read feed from the URL above.
